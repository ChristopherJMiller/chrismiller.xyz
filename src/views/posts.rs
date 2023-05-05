use super::TemplateHead;

markup::define! {
  Posts<'a>(title: &'a str) {
      @markup::doctype()
      html {
          @TemplateHead { title: "All Posts" }
          body {
              @Header { title }
              #main {
                  p {
                      "This domain is for use in illustrative examples in documents. You may \
                      use this domain in literature without prior coordination or asking for \
                      permission."
                  }
                  p {
                      a[href = "https://www.iana.org/domains/example"] {
                          "More information..."
                      }
                  }
              }
              @Footer { year: 2020 }
          }
      }
  }

  Header<'a>(title: &'a str) {
      header {
          h1 { @title }
      }
  }

  Footer(year: u32) {
      footer {
          "(c) " @year
      }
  }
}
