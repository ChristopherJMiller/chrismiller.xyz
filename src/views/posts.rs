use markup::DynRender;

use crate::models::posts::Post;

use super::Layout;

#[inline(always)]
pub fn render<'a>(posts: Vec<Post>) -> Layout<'a, DynRender<'a>> {
  Layout {
    location: "/posts",
    title: "All Posts",
    body: markup::new! {
      div[class="flex flex-row gap-2 my-3"] {
        h1[class="text-4xl underline pb-1"] {
          "All Posts"
        }

        a[class="underline italic text-lg self-end", href={"/rss.xml"}] {
          "Subscribe to RSS Feed"
        }
      }

      div {
        @for post in posts.iter() {
          div[class="flex flex-col mb-10"] {
            a[class="text-xl font-semibold underline", href={format!("/blog/{}", &post.post_url)}] {
              { &post.title }
            }
            p {
              { &post.format_date() }
            }
            p {
              { &post.description() }
            }
          }
        }
      }
    },
  }
}
