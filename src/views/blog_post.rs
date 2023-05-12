use markup::DynRender;

use super::Layout;
use crate::models::posts::Post;

#[inline(always)]
pub fn render<'a>(post: &'a Post) -> Layout<'a, DynRender<'a>> {
  Layout {
    location: "/blog",
    title: &post.title,
    body: markup::new! {
      div[class="my-6 flex flex-col justify-between"] {
        div[class="text-3xl font-bold underline"] {
          { &post.title }
        }
        p[class="text-xl font-light"] {
          { &post.format_date() }
        }
        div[class="text-serif my-4"] {
          @markup::raw(&post.body)
        }
      }
    },
  }
}
