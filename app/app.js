import Elm from "./Main.elm";

const elmDiv = document.querySelector("#elm-container");

if (elmDiv) {
	Elm.Main.embed(elmDiv);
}
