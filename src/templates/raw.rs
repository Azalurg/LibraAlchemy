pub const TEMP_MAIN: &str = "<!DOCTYPE html>
<html>

<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">

    <title>LibraAlchemy</title>
    <link rel=\"icon\" type=\"image/png\" href=\"https://raw.githubusercontent.com/Azalurg/public/main/images/icon.ico\">

    {{> style}}
</head>

<body>
    <navbar class=\"shadow\">
        <div style=\"font-size: 1.5rem;\">
            LibraAlchemy {{version}}
        </div>
    </navbar>

    <section class=\"card\">
        <h1>LibraAlchemy</h1>
        <p>Craft your private audio library with open-source magic. Curate,
            personalize, and transmute your collection offline, powered by alchemy.</p>
        <div class=\"btn-group\">
            <button class=\"btn active\" onclick=\"showGrid(1); activeButton(1)\" id=\"button1\">Books</button>
            <button class=\"btn\" onclick=\"showGrid(2); activeButton(2)\" id=\"button2\">Authors</button>
            <button class=\"btn\" onclick=\"showGrid(3); activeButton(3)\" id=\"button3\">Series</button>
        </div>
    </section>

    <section style=\"margin-top: 4rem;\">
        <div class=\"container grid-container visible\" id=\"grid1\">
            <h3>Books ({{books_amount}})</h3>
            <ul class=\"lib\">
                {{#each books}}
                {{> book}}
                {{/each}}
            </ul>
        </div>

        <div class=\"container grid-container\" id=\"grid2\">
            <h3 class=\"\">Authors ({{authors_amount}})</h3>
            <ul class=\"lib\">
                {{#each authors}}
                {{> author}}
                {{/each}}
            </ul>
        </div>

        <div class=\"container grid-container\" id=\"grid3\">
            <h3 class=\"\">Series ({{series_amount}})</h3>
            <ul class=\"lib\">
                {{#each series}}
                {{> series}}
                {{/each}}
            </ul>
        </div>
    </section>

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

</html>
";

pub const TEMP_AUTHOR: &str = "<li class=\"lib-cell shadow\">
{{#if cover}}
<img src=\"{{cover}}\" class=\"lib-img\" alt=\"{{name}}\">
{{else}}
<img src=\"https://raw.githubusercontent.com/Azalurg/ShadowTube/master/images/128.png\" class=\"lib-img\" alt=\"{{name}}\">
{{/if}}

<div class=\"lib-text lib-tex-first\" title=\"{{name}}\">{{name}}</div>

</li>
";

pub const TEMP_SERIES: &str = "<li class=\"lib-cell shadow\">
{{#if cover}}
<img src=\"{{cover}}\" class=\"lib-img\" alt=\"{{title}}\">
{{else}}
<img src=\"https://raw.githubusercontent.com/Azalurg/ShadowTube/master/images/128.png\" class=\"lib-img\" alt=\"{{title}}\">
{{/if}}

<div class=\"lib-text lib-tex-first\" title=\"{{title}}\">{{title}}</div>
<div class=\"lib-text lib-text-second\" title=\"{{author}}\">{{author}}</div>

</li>
";

pub const TEMP_BOOK: &str = "<li class=\"lib-cell shadow\">
{{#if cover}}
<img src=\"{{cover}}\" class=\"lib-img\" alt=\"{{title}}\">
{{else}}
<img src=\"https://raw.githubusercontent.com/Azalurg/ShadowTube/master/images/128.png\" class=\"lib-img\" alt=\"{{title}}\">
{{/if}}

<div class=\"lib-text lib-tex-first\" title=\"{{title}}\">{{title}}</div>
<div class=\"lib-text lib-text-second\" title=\"{{author}}\">{{author}}</div>

</li>
";

pub const TEMP_STYLE: &str = "<style>
:root {
    --color-bg: #0d1117;
    --color-bg-alt: #161b22;
    --color-text: #e6edf3;
    --color-text-alt: #7d8590;

    --color-navbar-bg: #010409;
    --color-navbar-text: #c9d1d9;

    --color-accent-1: hsl(160, 99%, 55%);
    --color-accent-2: hsl(163, 90%, 38%);
    --color-accent-3: hsl(166, 85%, 21%);

    --aspect-ratio: calc(1.38);
}

.btn,body{color:var(--color-text)}.card,.lib-cell{background-color:var(--color-bg-alt)}.card,.lib-text{text-align:center}body{margin:0;line-height:1;min-height:100vh;font-weight:400;font-family:Calibri,sans-serif;background-color:var(--color-bg)}.container{max-width:93%;margin-right:auto;margin-left:auto}navbar{position:relative;display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;padding:.75rem 1.5rem;background-color:var(--color-navbar-bg);color:var(--color-navbar-text)}.shadow{transition:.3s;box-shadow:0 .5rem 1rem hsl(166,85%,21%,.5)!important}.card{padding:6rem 3rem}.card>p{font-size:1.2rem;width:50%;margin:1rem auto}.grid-container{display:none}.visible{display:block}.lib{width:100%;margin-top:2rem;list-style:none;padding:0;display:flex;flex-wrap:wrap;align-items:center;justify-content:center}.lib-cell{width:160px;height:calc(160px * var(--aspect-ratio));margin:1rem;border-radius:.5rem}.h1,.h2,.h3,.h4,.h5,.h6,h1,h2,h3,h4,h5,h6{margin-top:0}.lib-img{width:100%;aspect-ratio:1/1;border-radius:.5rem .5rem 0 0;filter:brightness(.8)}.lib-img:hover{filter:brightness(1)}.lib-text{overflow:hidden;padding:.05rem .5rem;text-overflow:ellipsis;white-space:nowrap}.lib-tex-first{padding-top:.3rem;font-size:.9rem}.lib-text-second{padding-top:.15rem;font-size:.8rem;color:var(--color-text-alt)}.h1,h1{font-size:2.5rem;line-height:1.1em}.h2,h2{font-size:2rem;line-height:1.2em}.h3,h3{font-size:1.65rem;line-height:1.3em}.h4,h4{font-size:1.4rem;line-height:1.4em}.h5,h5{font-size:1.1rem;line-height:1.5em}.h6,h6{font-size:.9rem;line-height:1.6em}.btn{font-size:1.2rem;background-color:transparent;border:.2rem solid var(--color-accent-1);border-radius:1rem;padding:.5rem 1rem;margin:.5rem;color:var(--color-text-alt);border-color:var(--color-accent-2)}.btn:focus,.btn:hover{border-color:var(--color-accent-1)}.active{background-color:var(--color-bg-alt);border-color:var(--color-accent-3);color:var(--color-text)}
</style>
";
