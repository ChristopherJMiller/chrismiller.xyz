import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import { getPostList } from "../blog/api";
import { BlogEntry } from "../blog/entry";
import { PostListing } from "../blog/post";

const App = () => {
  const [loading, setLoading] = useState<boolean>(true);
  const [latestPost, setLatestPost] = useState<PostListing | null>(null);

  useEffect(() => {
    const runGetPost = async () => {
      try {
        const posts = await getPostList();
        setLatestPost(posts[0]);
      } finally {
        setLoading(false);
      }
    };

    runGetPost();
  }, []);
  
  const element = !loading ? latestPost ? <BlogEntry post={latestPost} /> : "Oops, something went wrong loading the post. Please try refreshing." : null;

  return element;
}

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
