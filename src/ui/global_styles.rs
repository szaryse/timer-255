pub fn global_styles() -> String {
    r"<style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        html {
            font-family: 'Consolas', sans-serif;
        }
        body {
            background-color: transparent;
            color: #adadb8;
        }
        svg:hover path {
            fill: #00FF00;
        }
    </style>".to_string()
}
