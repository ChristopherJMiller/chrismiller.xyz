use markup::DynRender;

use super::Layout;

#[inline(always)]
pub fn render<'a>() -> Layout<'a, DynRender<'a>> {
  Layout {
    location: "/posts",
    title: "All Posts",
    body: markup::new! {
      h1 { "Hello Posts!" }
    }
  }
}