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
import List.Extra

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
  , currentChannel: Maybe Channel
  , currentError: Maybe Http.Error
  , currentDate: Date
  }


init : (Model, Cmd Msg)
init =
  ( Model [] [] Nothing Nothing Nothing (Date.fromTime 0)
  , Cmd.batch [getNow, fetchChannels, fetchItems]
  )

-- UPDATE

type Msg
  = FetchChannels
  | GotChannels (Result Http.Error (List Channel))
  | FetchItems
  | GotItems (Result Http.Error (List Item))
  | SelectItem Item
  | UnselectItem
  | SelectChannel Channel
  | UnselectChannel
  | ReadAllItems
  | DidReadItems (Result Http.Error ())
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

    FetchItems ->
      ({model | currentError = Nothing}, fetchItems)

    GotItems (Ok items) ->
      ({model | items = List.sortWith compareItems <| List.map (addChannelToItem model.channels) items, currentError = Nothing}, Cmd.none)

    GotItems (Err err) ->
      ({model | currentError = Just err}, Cmd.none)

    SelectItem item ->
      let i =
        {item | read = True}
      in
        ({model | currentItem = Just i, items = updateItem model.items i, currentError = Nothing}, readItem i)

    UnselectItem ->
      ({model | currentItem = Nothing, currentError = Nothing}, Cmd.none)

    SelectChannel c ->
      ({model | currentChannel = Just c, currentError = Nothing}, Cmd.none)

    UnselectChannel ->
      ({model | currentChannel = Nothing, currentError = Nothing}, Cmd.none)

    ReadAllItems ->
      ({model | currentItem = Nothing, items = List.map (\i -> {i | read = True}) model.items, currentError = Nothing}, readAllItems)

    DidReadItems (Ok ()) ->
      ({model | currentError = Nothing}, Cmd.none)

    DidReadItems (Err err) ->
      ({model | currentError = Just err}, Cmd.none)

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
  a [ href "#", onClick <| SelectChannel c] [ text c.title ]

viewChannels: List Channel -> Html Msg
viewChannels lc =
  div []
    <| List.append [ a [ href "#", onClick UnselectChannel ] [ text "All Feeds" ] ]
    <| List.map viewChannel lc

viewItem: Date -> Maybe Item -> Item -> Html Msg
viewItem now cur i =
  if cur == Just i then
    viewItemSelectItem i
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
  div
  [ classList [ ("brief", True) , ("brief-read", i.read) ]
  , onClick <| SelectItem i
  ]
  [ span [ class "brief-channel" ] [ text <| Maybe.withDefault "" <| Maybe.map .title i.channel ]
  , span [ class "brief-title" ] [ text i.title ]
  , span [ class "brief-date" ] [text <|  showInterval now i.pub_date]
  ]

viewItemSelectItem: Item -> Html Msg
viewItemSelectItem i =
  div [ class "details-content" ]
  [ div [class "details-bar"]
    [ span [onClick UnselectItem] [text "X"]
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

viewItems: List Item -> Maybe Item -> Maybe Channel -> Date -> Html Msg
viewItems li mi mc d =
  div []
    <| List.map (viewItem d mi)
    <| List.filter (\i -> mc == Nothing || mc == i.channel) li

viewError: Http.Error -> Html Msg
viewError e =
  div [style [("background-color","red")]]
  [ text <| toString e
  ]

viewToolbar: Html Msg
viewToolbar=
  div []
  [ a [ class "site-toolbar-read-all", href "#", onClick ReadAllItems] []
  , a [ class "site-toolbar-refresh", href "#", onClick FetchItems] []
  ]


view: Model -> Html Msg
view model =
  div [ id "site" ]
  [ header [ class "site-header" ] [ text "Feeder" ]
  , nav [ class "site-nav" ] [ viewChannels model.channels ]
  , div [ class "site-toolbar" ] [ viewToolbar]
  , main_ [ class "site-main" ]
    [ viewItems model.items model.currentItem model.currentChannel model.currentDate
    ]
  , footer [ class "site-footer" ] [ viewMaybe viewError <| model.currentError ]
  ]

-- SUBSCRIPTIONS

subscriptions: Model -> Sub Msg
subscriptions model =
  Time.every Time.minute UpdateDate

-- HTTP

baseUrl: String
--baseUrl = "http://localhost:8888"
baseUrl = "/api"

type alias Channel =
  { id : Int
  , title : String
  , link : String
  , description : String
  , source : String
  , image : Maybe String
  , ttl : Maybe Int
  }

type alias Item =
  { id : Int
  , channel_id: Int
  , title: String
  , link: String
  , description: String
  , pub_date: Date
  , read: Bool
  , channel: Maybe Channel
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

itemDecoder: Decode.Decoder Item
itemDecoder =
  Pipeline.decode Item
    |> Pipeline.required "id" Decode.int
    |> Pipeline.required "channel_id" Decode.int
    |> Pipeline.required "title" Decode.string
    |> Pipeline.required "link" Decode.string
    |> Pipeline.required "description" Decode.string
    |> Pipeline.required "pub_date" DecodeExtra.date
    |> Pipeline.required "read" Decode.bool
    |> Pipeline.hardcoded Nothing

decodeItems: Decode.Decoder (List Item)
decodeItems =
  Decode.list itemDecoder


getChannelById: List Channel -> Int -> Maybe Channel
getChannelById cl id =
  List.Extra.find (\c -> c.id == id) cl

addChannelToItem: List Channel -> Item -> Item
addChannelToItem cl i =
  { i | channel = getChannelById cl i.channel_id }

compareItems: Item -> Item -> Order
compareItems i1 i2 =
  DateExtra.compare i2.pub_date i1.pub_date

updateItem: List Item -> Item -> List Item
updateItem li i =
  let updateItem u =
    if u.id == i.id then
      i
    else
      u
  in
    List.map updateItem li

fetchChannels : Cmd Msg
fetchChannels =
  let
    url =
      baseUrl ++ "/channels"
  in
    Http.send GotChannels (Http.get url decodeChannels)

fetchItems : Cmd Msg
fetchItems =
  let
    url =
      baseUrl ++ "/items?max_items=-1&from_id=-1&to_id=-1"
  in
    Http.send GotItems (Http.get url decodeItems)

emptyPost : String -> Http.Request ()
-- Because when Http.post expects a JSON, and empty responses aren't valid JSON
-- we have to use this to get around it.
emptyPost url =
    Http.request
       { method = "POST"
       , headers = []
       , url = url
       , body = Http.emptyBody
       , expect = Http.expectStringResponse (\_ -> Ok ())
       , timeout = Nothing
       , withCredentials = False
       }

readItem : Item -> Cmd Msg
readItem i =
  let
    url =
      baseUrl ++ "/read/" ++ (toString i.id)
  in
    Http.send DidReadItems (emptyPost url)

readAllItems : Cmd Msg
readAllItems =
  let
    url =
      baseUrl ++ "/read/all"
  in
    Http.send DidReadItems (emptyPost url)
