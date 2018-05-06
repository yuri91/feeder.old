module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as Decode
import Json.Decode.Pipeline as Pipeline


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
  }


init : (Model, Cmd Msg)
init =
  ( Model [] [] Nothing Nothing
  , fetchChannels
  )

-- UPDATE

type Msg
  = FetchChannels
  | GotChannels (Result Http.Error (List Channel))
  | FetchItems Int
  | GotItems (Result Http.Error (List Item))
  | Details Item

update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    FetchChannels ->
      ({model | currentError = Nothing}, fetchChannels)

    GotChannels (Ok channels) ->
      ({model | channels = channels, currentError = Nothing}, Cmd.none)

    GotChannels (Err err) ->
      ({model | currentError = Just err}, Cmd.none)

    FetchItems it ->
      ({model | currentError = Nothing}, fetchItems it)

    GotItems (Ok items) ->
      ({model | items = items, currentError = Nothing}, Cmd.none)

    GotItems (Err err) ->
      ({model | currentError = Just err}, Cmd.none)

    Details item ->
      ({model | currentItem = Just item, currentError = Nothing}, Cmd.none)

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
  a [ href "#" , onClick <| FetchItems c.id] [ text c.title ]


viewItem: Item -> Html Msg
viewItem i =
  div []
  [ div [ class "title" , onClick <| Details i ]
    [ text <| Maybe.withDefault "[no title]" i.title
    ]
  ]

viewItemDetail: Item -> Html Msg
viewItemDetail i =
  div [ class "details-content" ]
  [ h4 []
    [ a [ href <| Maybe.withDefault "" i.link ]
      [ text <| Maybe.withDefault "[no title]" i.title
      ]
    ]
  , div [ class "details-iframe-container" ]
    [ iframe
      [ class "details-iframe"
      , sandbox ""
      , srcdoc <| Maybe.withDefault "" i.description
      ] []
    ]
  , footer []
    [ div [] [ viewMaybe text <| i.pub_date ]
    , div [] [ viewMaybe text <| i.author ]
    , div [] [ viewMaybe text <| i.guid ]
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
  , section [ class "site-details" ] [ viewMaybe viewItemDetail <| model.currentItem ]
  , main_ [ class "site-main" ] <| List.map viewItem model.items
  , footer [ class "site-footer" ] [ viewMaybe viewError <| model.currentError ]
  ]

-- SUBSCRIPTIONS

subscriptions: Model -> Sub Msg
subscriptions model =
  Sub.none

-- HTTP

baseUrl: String
baseUrl = "http://localhost:8888"

type alias Channel =
  { id : Int
  , title : String
  , link : String
  , description : String
  , source : String
  , language : Maybe String
  , copyright : Maybe String
  , pub_date : Maybe String
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
    |> Pipeline.optional "language" (Decode.nullable Decode.string) Nothing
    |> Pipeline.optional "copyright" (Decode.nullable Decode.string) Nothing
    |> Pipeline.optional "pub_date" (Decode.nullable Decode.string) Nothing
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
  , title: Maybe String
  , link: Maybe String
  , description: Maybe String
  , author: Maybe String
  , guid: Maybe String
  , pub_date: Maybe String
  }

fetchItems : Int -> Cmd Msg
fetchItems id =
  let
    url =
      baseUrl ++ "/items/" ++ (toString id)
  in
    Http.send GotItems (Http.get url decodeItems)

itemDecoder: Decode.Decoder Item
itemDecoder =
  Pipeline.decode Item
    |> Pipeline.required "id" Decode.int
    |> Pipeline.required "channel_id" Decode.int
    |> Pipeline.optional "title" (Decode.nullable Decode.string) Nothing
    |> Pipeline.optional "link" (Decode.nullable Decode.string) Nothing
    |> Pipeline.optional "description" (Decode.nullable Decode.string) Nothing
    |> Pipeline.optional "author" (Decode.nullable Decode.string) Nothing
    |> Pipeline.optional "guid" (Decode.nullable Decode.string) Nothing
    |> Pipeline.optional "pub_date" (Decode.nullable Decode.string) Nothing

decodeItems: Decode.Decoder (List Item)
decodeItems =
  Decode.list itemDecoder
