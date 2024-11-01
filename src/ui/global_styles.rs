pub fn global_styles() -> &'static str {
    r"
        <style>
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }
            html {
                font-family: 'Consolas', sans-serif;
            }
            svg:hover path {
                fill: #00FF00;
            }
        </style>"
}
