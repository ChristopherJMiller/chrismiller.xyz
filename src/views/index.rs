use markup::DynRender;

use super::Layout;

#[inline(always)]
pub fn render<'a>() -> Layout<'a, DynRender<'a>> {
  Layout {
    location: "/",
    title: "Home",
    body: markup::new! {
      h1[class="text-4xl font-bold text-center mt-8"] { 
        "Chris Miller" 
      }
      p[class="text-center italic"] {
        "Computer Engineer / UX Designer / Maker"
      }
      div[class="text-center"] {
        a[href="https://github.com/ChristopherJMiller", class="underline mx-1"] {
          "Github"
        }
        a[href="https://www.linkedin.com/in/christopherjmill", class="underline mx-1"] {
          "LinkedIn"
        }
      }
    }
  }
}