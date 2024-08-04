extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;

#[test]
fn test_subexpression() {
    let hbs = Handlebars::new();

    let data = json!({"a": 1, "b": [{"n" : "n1"}, {"n": "n2"}]});

    assert_eq!(
        hbs.render_template("{{a}} {{#each b as |ar|}}{{a}}{{ar.n}} {{/each}}", &data)
            .unwrap(),
        "1 1n1 1n2 "
    );
}
