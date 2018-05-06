module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as Decode
import Json.Encode as Encode
import Json.Decode.Pipeline as Pipeline
import Json.Decode.Extra as DecodeExtra
import Date exposing (..)
import Date.Extra as DateExtra
import Time exposing (..)
import Task exposing (..)

main =
  Html.program
    { init = init
    , view = view
    , update = update
    , subscriptions = subscriptions
    }


-- MODEL

type alias Model =
  { channels : List Channel
  , items : List Item
  , currentItem: Maybe Item
  , currentError: Maybe Http.Error
  , currentDate: Date
  }


init : (Model, Cmd Msg)
init =
  ( Model [] [] Nothing Nothing (Date.fromTime 0)
  , Cmd.batch [getNow, fetchChannels]
  )

-- UPDATE

type Msg
  = FetchChannels
  | GotChannels (Result Http.Error (List Channel))
  | FetchItems Channel
  | GotItems (Result Http.Error (List Item))
  | Details Item
  | CloseDetails
  | UpdateDate Time

update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    FetchChannels ->
      ({model | currentError = Nothing}, fetchChannels)

    GotChannels (Ok channels) ->
      ({model | channels = channels, currentError = Nothing}, Cmd.none)

    GotChannels (Err err) ->
      ({model | currentError = Just err}, Cmd.none)

    FetchItems c ->
      ({model | currentError = Nothing}, fetchItems c)

    GotItems (Ok items) ->
      ({model | items = List.sortWith compareItems <| items, currentError = Nothing}, Cmd.none)

    GotItems (Err err) ->
      ({model | currentError = Just err}, Cmd.none)

    Details item ->
      ({model | currentItem = Just item, currentError = Nothing}, Cmd.none)

    CloseDetails ->
      ({model | currentItem = Nothing, currentError = Nothing}, Cmd.none)

    UpdateDate t ->
      ({model | currentDate = Date.fromTime <| t}, Cmd.none)


getNow: Cmd Msg
getNow =
  Time.now
    |> Task.perform UpdateDate

-- VIEW

viewMaybe: (a -> Html b) -> Maybe a -> Html b
viewMaybe view m =
  case m of
    Just k ->
      view k
    Nothing ->
      text ""


viewChannel: Channel -> Html Msg
viewChannel c =
  a [ href "#" , onClick <| FetchItems c] [ text c.title ]


viewItem: Date -> Maybe Item -> Item -> Html Msg
viewItem now cur i =
  if cur == Just i then
    viewItemDetails i
  else
    viewItemBrief i now

showInterval: Date -> Date -> String
showInterval cur_date item_date =
  let month =
    DateExtra.diff DateExtra.Month item_date cur_date
  in
    if month /= 0 then
      (toString <| month) ++ "mon"
    else
      let day =
        DateExtra.diff DateExtra.Day item_date cur_date
      in
        if day /= 0 then
          (toString <| day) ++ "d"
        else
          let hour =
            DateExtra.diff DateExtra.Hour item_date cur_date
          in
            if hour /= 0 then
              (toString <| hour) ++ "h"
            else
              let min =
                DateExtra.diff DateExtra.Minute item_date cur_date
              in
                (toString <| min) ++ "min"


viewItemBrief: Item -> Date -> Html Msg
viewItemBrief i now =
  div [ class "brief" , onClick <| Details i ]
  [ span [ class "brief-channel" ] [ text i.channel.title ]
  , span [ class "brief-title" ] [ text i.title ]
  , span [ class "brief-date" ] [text <|  showInterval now i.pub_date]
  ]

viewItemDetails: Item -> Html Msg
viewItemDetails i =
  div [ class "details-content" ]
  [ div [class "details-bar"]
    [ span [onClick CloseDetails] [text "X"]
    ]
  , h4 []
    [ a [ href i.link ]
      [ text  i.title
      ]
    ]
  , div [ class "details-description" ]
    [ p [property "innerHTML" <| Encode.string i.description] []
    ]
  , footer [ class "details-footer" ]
    [ text <| toString i.pub_date 
    ]
  ]

viewError: Http.Error -> Html Msg
viewError e =
  div [style [("background-color","red")]]
  [ text <| toString e
  ]

view: Model -> Html Msg
view model =
  div [ id "site" ]
  [ header [ class "site-header" ] [ text "Feeder" ]
  , nav [ class "site-nav" ] <| List.map viewChannel model.channels
  , main_ [ class "site-main" ] <| List.map (viewItem model.currentDate model.currentItem) model.items
  , footer [ class "site-footer" ] [ viewMaybe viewError <| model.currentError ]
  ]

-- SUBSCRIPTIONS

subscriptions: Model -> Sub Msg
subscriptions model =
  Time.every Time.minute UpdateDate

-- HTTP

baseUrl: String
baseUrl = "http://localhost:8888"

type alias Channel =
  { id : Int
  , title : String
  , link : String
  , description : String
  , source : String
  , image : Maybe String
  , ttl : Maybe Int
  }

channelDecoder: Decode.Decoder Channel
channelDecoder =
  Pipeline.decode Channel
    |> Pipeline.required "id" Decode.int
    |> Pipeline.required "title" Decode.string
    |> Pipeline.required "link" Decode.string
    |> Pipeline.required "description" Decode.string
    |> Pipeline.required "source" Decode.string
    |> Pipeline.optional "image" (Decode.nullable Decode.string) Nothing
    |> Pipeline.optional "ttl" (Decode.nullable Decode.int) Nothing

decodeChannels: Decode.Decoder (List Channel)
decodeChannels =
  Decode.list channelDecoder

fetchChannels : Cmd Msg
fetchChannels =
  let
    url =
      baseUrl ++ "/channels"
  in
    Http.send GotChannels (Http.get url decodeChannels)

type alias Item =
  { id : Int
  , channel_id: Int
  , title: String
  , link: String
  , description: String
  , pub_date: Date
  , channel: Channel
  }

fetchItems : Channel -> Cmd Msg
fetchItems c =
  let
    url =
      baseUrl ++ "/items/" ++ (toString c.id)
  in
    Http.send GotItems (Http.get url <| decodeItems c)

itemDecoder: Channel -> Decode.Decoder Item
itemDecoder c =
  Pipeline.decode Item
    |> Pipeline.required "id" Decode.int
    |> Pipeline.required "channel_id" Decode.int
    |> Pipeline.required "title" Decode.string
    |> Pipeline.required "link" Decode.string
    |> Pipeline.required "description" Decode.string
    |> Pipeline.required "pub_date" DecodeExtra.date
    |> Pipeline.hardcoded c

decodeItems: Channel -> Decode.Decoder (List Item)
decodeItems c =
  Decode.list <| itemDecoder c

compareItems: Item -> Item -> Order
compareItems i1 i2 =
  DateExtra.compare i2.pub_date i1.pub_date
