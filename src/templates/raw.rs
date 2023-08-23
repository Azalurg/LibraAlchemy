pub const TEMP_MAIN: &str = "<!DOCTYPE html>
<html>

<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">

    <title>LibraAlchemy</title>
    <link rel=\"icon\" type=\"image/png\" href=\"https://raw.githubusercontent.com/Azalurg/public/main/images/icon.ico\">

    <link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.1/dist/css/bootstrap.min.css\" rel=\"stylesheet\"
        integrity=\"sha384-4bw+/aepP/YC94hEpVNVgiZdgIC5+VKNBQNGCHeKRQN+PtmoHDEXuppvnDJzQIu9\" crossorigin=\"anonymous\">
    <script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.1/dist/js/bootstrap.bundle.min.js\"
        integrity=\"sha384-HwwvtgBNo3bZJJLYd8oVXjrBZt8cqVSpeBNS5n7C8IVInixGAoxmnlMuBnhbgrkm\"
        crossorigin=\"anonymous\"></script>

    <style>
        .cover {
            aspect-ratio: 1/1;
            width: 100%;
        }

        @media (min-width:1200px) {
            .row-cols-xl-8>* {
                flex: 0 0 auto;
                width: 12.5%
            }
        }

        .grid-container {
            display: none;
        }

        .visible {
            display: block;
        }

        .cardText {
            overflow: hidden;
            padding: 0.06em 0.5em;
            text-align: left;
            text-overflow: ellipsis;
            white-space: nowrap;
            text-align: center;
        }

        .cardText-first {
            padding-top: 0.24em;
            font-size: 0.9rem;
        }
        .cardText-second {
            font-size: 0.83rem;
        }

    </style>

    </style>

</head>

<body>
    <header>
        <div class=\"navbar navbar-dark bg-dark shadow-sm\">
            <div class=\"container\">
                <a href=\"/\" class=\"navbar-brand d-flex align-items-center\">
                    <strong>LibraAlchemy {{version}}</strong>
                </a>
            </div>
        </div>
    </header>
    <main>
        <section class=\"py-5 container text-center\">
            <div class=\"row py-lg-5\">
                <div class=\"col-lg-6 col-md-8 mx-auto\">
                    <h1 class=\"fw-light\">LibraAlchemy</h1>
                    <p class=\"lead text-muted\">Craft your private audio library with open-source magic. Curate,
                        personalize, and transmute your collection offline, powered by alchemy.</p>
                    <div class=\"btn-group mb-3\">
                        <button class=\"btn btn-primary active\" onclick=\"showGrid(1); activeButton(1)\"
                            id=\"button1\">Books</button>
                        <button class=\"btn btn-primary\" onclick=\"showGrid(2); activeButton(2)\"
                            id=\"button2\">Authors</button>
                        <button class=\"btn btn-primary\" onclick=\"showGrid(3); activeButton(3)\"
                            id=\"button3\">Series</button>
                    </div>
                </div>
            </div>
        </section>
    </main>

    <div class=\"py-5 bg-body-tertiary\">
        <div class=\"container grid-container visible\" id=\"grid1\">
            <h3 class=\"pb-2\">Books ({{books_amount}})</h3>
            <div class=\"row row-cols-2 row-cols-sm-3 row-cols-md-4 row-cols-lg-6 row-cols-xl-8 g-3\">
                {{#each books}}
                {{> book}}
                {{/each}}
            </div>
        </div>

        <div class=\"container grid-container\" id=\"grid2\">
            <h3 class=\"pb-2\">Authors ({{authors_amount}})</h3>
            <div class=\"row row-cols-2 row-cols-sm-3 row-cols-md-4 row-cols-lg-6 row-cols-xl-8 g-3\">
                {{#each authors}}
                {{> author}}
                {{/each}}
            </div>
        </div>

        <div class=\"container grid-container\" id=\"grid3\">
            <h3 class=\"pb-2\">Series ({{series_amount}})</h3>
            <div class=\"row row-cols-2 row-cols-sm-3 row-cols-md-4 row-cols-lg-6 row-cols-xl-8 g-3\">
                {{#each series}}
                {{> series}}
                {{/each}}
            </div>
        </div>

        <script>
            function showGrid(gridNumber) {
                // Hide all grid containers
                const gridContainers = document.querySelectorAll('.grid-container');
                gridContainers.forEach(container => {
                    container.classList.remove('visible');
                });

                // Show the selected grid container
                const selectedGrid = document.getElementById(`grid${gridNumber}`);
                selectedGrid.classList.add('visible');
            }

            function activeButton(buttonNumber) {
                const buttons = document.querySelectorAll('.btn');
                buttons.forEach(button => {
                    button.classList.remove('active');
                });

                const selectedButton = document.getElementById(`button${buttonNumber}`);
                selectedButton.classList.add('active');
            }
        </script>
</body>

</html>";

pub const TEMP_AUTHOR: &str = "<div class=\"col\">
<div class=\"card shadow-sm\">
    {{#if cover}}
        <img src=\"{{cover}}\" class=\"bd-placeholder-img card-img-top cover\" alt=\"{{name}}\" aria-label=\"Placeholder: {{name}}\">
    {{else}}
        <img src=\"https://raw.githubusercontent.com/Azalurg/ShadowTube/master/images/128.png\" class=\"bd-placeholder-img card-img-top cover\" alt=\"{{title}}\" aria-label=\"Placeholder: {{title}}\">
    {{/if}}
    <div>
        <div class=\"cardText cardText-first\">{{name}}</div>
    </div>
</div>
</div>
";

pub const TEMP_SERIES: &str = "<div class=\"col\">
<div class=\"card shadow-sm\">
    {{#if cover}}
        <img src=\"{{cover}}\" class=\"bd-placeholder-img card-img-top cover\" alt=\"{{title}}\" aria-label=\"Placeholder: {{title}}\">
    {{else}}
        <img src=\"https://raw.githubusercontent.com/Azalurg/ShadowTube/master/images/128.png\" class=\"bd-placeholder-img card-img-top cover\" alt=\"{{title}}\" aria-label=\"Placeholder: {{title}}\">
    {{/if}}
    <div>
        <div class=\"cardText cardText-first\">{{title}}</div>
        <div class=\"cardText cardText-second\">{{author}}</div>
    </div>
</div>
</div>
";

pub const TEMP_BOOK: &str = "<div class=\"col\">
<div class=\"card shadow-sm\">
    {{#if cover}}
        <img src=\"{{cover}}\" class=\"bd-placeholder-img card-img-top cover\" alt=\"{{title}}\" aria-label=\"Placeholder: {{title}}\">
    {{else}}
        <img src=\"https://raw.githubusercontent.com/Azalurg/ShadowTube/master/images/128.png\" class=\"bd-placeholder-img card-img-top cover\" alt=\"{{title}}\" aria-label=\"Placeholder: {{title}}\">
    {{/if}}
    <div>
        <div class=\"cardText cardText-first\">{{title}}</div>
        <div class=\"cardText cardText-second\">{{author}}</div>
    </div>
</div>
</div>
";
