pub const TEMP_MAIN: &str = "<!DOCTYPE html>
<html>
<head>
    <title>Library</title>
    <link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.1/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-4bw+/aepP/YC94hEpVNVgiZdgIC5+VKNBQNGCHeKRQN+PtmoHDEXuppvnDJzQIu9\" crossorigin=\"anonymous\">
    <script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.1/dist/js/bootstrap.bundle.min.js\" integrity=\"sha384-HwwvtgBNo3bZJJLYd8oVXjrBZt8cqVSpeBNS5n7C8IVInixGAoxmnlMuBnhbgrkm\" crossorigin=\"anonymous\"></script>
</head>
<body>
    <header>
        <div class=\"navbar navbar-dark bg-dark shadow-sm\">
            <div class=\"container\">
                <a href=\"/\" class=\"navbar-brand d-flex align-items-center\">
                    <strong>Library</strong>
                </a>
            </div>
        </div>
    </header>
    <main>
        <section class=\"py-5 container text-center\">
            <div class=\"row py-lg-5\">
                <div class=\"col-lg-6 col-md-8 mx-auto\">
                    <h1 class=\"fw-light\">Library</h1>
                    <p class=\"lead text-muted\">A simple book library</p>
                    <p>
                        <a href=\"/books\" class=\"btn btn-primary my-2\">View Books</a>
                        <a href=\"/authors\" class=\"btn btn-secondary my-2\">View Authors</a>
                    </p>
                </div>
            </div>
        </section>
    </main>
    <div class=\"album py-5 bg-body-tertiary\">
        <div class=\"container\">
            <div class=\"row row-cols-2 row-cols-sm-4 row-cols-md-6 g-3\">
                {{#each authors}}
                    {{> author}}
                {{/each}}
            </div>
        </div>
    </div>
</body>
</html>
";

pub const TEMP_AUTHOR: &str = "{{#if books}}
{{#each books}}
    {{> book}}
{{/each}}
{{/if}}

{{#if albums}}
{{#each albums}}
    {{> album}}
{{/each}}
{{/if}}
";

pub const TEMP_ALBUM: &str = "{{#if books}}
{{#each books}}
    {{> book}}
{{/each}}
{{/if}}
";

pub const TEMP_BOOK: &str = "<div class=\"col\">
<div class=\"card shadow-sm\">
    <img src=\"{{cover}}\" class=\"bd-placeholder-img card-img-top\" style=\"max-width: 10rem; max-height: 10rem\" alt=\"{{title}}\" aria-label=\"Placeholder: {{title}}\">
    <div class=\"card-body\">
        <p class=\"card-text\">{{title}}</p>
    </div>
</div>
</div>
";
