exports.config = {
	files: {
		javascripts: {
			joinTo: "js/app.js"
		},
		stylesheets: {
			joinTo: "css/app.css"
		}
	},
	conventions: {
		assets: /^(static)/
	},
	paths: {
		watched: ["static", "css", "js", "vendor", "elm"],
		public: "../public"
	},
	plugins: {
		babel: {
			ignore: [/vendor/]
		},
		elmBrunch: {
			elmFolder: "elm",
			mainModules: ["Main.elm"],
			outputFolder: "../js"
		}
	},
	modules: {
		autoRequire: {
			"js/app.js": ["js/app"]
		}
	}
};
