pub mod posts;
pub mod index;
pub mod blog_post;

markup::define! {
  NavItem<'a>(location: &'a str, title: &'a str, path: &'a str) {
    @if location == path {
      a[href=path, class="underline"] {
        @title
      }
    } else {
      a[href=path] {
        @title
      }
    }
  }
}

markup::define! {
  Layout<'a, Body: markup::Render>(location: &'a str, title: &'a str, body: Body) {
    @markup::doctype()
    html[lang="en"] {
      head {
        title { "Chris Miller - " @title }
        meta[charset="utf-8"];
        meta[name="description", content="Website of Christopher Miller"];
        link[rel="icon", href="/public/favicon.png"];
        link[rel="stylesheet", href="/assets/main.css"];
        link[rel="me", href="https://tech.lgbt/@alumux"];
      }
      body[class="bg-stone-50 dark:bg-stone-800 text-black dark:text-slate-200"] {
        div[class="container mx-auto my-2"] {
          div[class="flex justify-center py-4"] {
            a[href="/"] {
              img[class="object-none object-center rounded", src="/public/img/me.jpg", alt="Photo of Chris Miller"];
            }
          }
          div[class="flex flex-row justify-center gap-6"] {
            @for navlink in [
              NavItem { location, title: "Home", path: "/" },
              NavItem { location, title: "Blog", path: "/posts" },
            ].iter() {
              @navlink
            }
            a[href="https://raw.githubusercontent.com/ChristopherJMiller/Resume/publish/resume.pdf"] {
              "Resume"
            }
          }
          @body
        }
      }
    }
  }
}
