use markup::DynRender;

use super::Layout;
use crate::models::posts::Post;

#[inline(always)]
pub fn render<'a>(posts: Vec<Post>) -> Layout<'a, DynRender<'a>> {
  Layout {
    location: "/posts",
    title: "All Posts",
    body: markup::new! {
      h1[class="text-4xl underline my-3"] {
        "All Posts"
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
