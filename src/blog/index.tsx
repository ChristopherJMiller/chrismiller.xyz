import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import { getPost, getPostList } from "./api";
import { Post, PostListing } from "./post";

import {unified} from 'unified'
import remarkParse from 'remark-parse'
import remarkGfm from 'remark-gfm'
import remarkRehype from 'remark-rehype'
import rehypeStringify from 'rehype-stringify'

const App = () => {
  const [postListings, setPostListings] = useState<PostListing[] | null>(null);
  const [post, setPost] = useState<Post | null>(null);
  const [postContent, setPostContent] = useState<string>();
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const postParam: string | null = urlParams.get("post");


  useEffect(() => {
    const processP = async () => {
      console.log(post!!.content)
      const result = await unified()
        .use(remarkParse)
        .use(remarkGfm)
        .use(remarkRehype)
        .use(rehypeStringify)
        .process(post!!.content)

      const classesApplied = result
        .toString()
        .replaceAll("<p>", `<p class="my-4 text-serif">`)
        .replaceAll("<h1>", `<h1 class="mt-3 text-2xl text-semibold font-medium">`)
        .replaceAll("<h2>", `<h2 class="mt-3 text-xl text-semibold font-medium">`)
        
      setPostContent(classesApplied);
    }

    if (post) {
      processP();
    }
  }, [post]);

  useEffect(() => {
    if (postParam) {
      getPost(postParam).then(setPost);
    } else {
      getPostList().then(setPostListings);
    }
  }, []);

  console.log(postListings)

  const loading = postListings || post ? null : <div className="text-xl text-center my-5">Loading...</div>;

  const blogHeader = postListings ? <div className="text-3xl font-bold underline">Blog</div> : null;
  const blogEntries = postListings ? postListings.map((post) => 
    <div className="flex flex-col my-10">
      <a href={`/blog.html?post=${post.path}`} className="text-xl font-semibold underline">{post.name}</a>
      <div>{post.description}</div>
    </div>
  ) : null;  

  const blogPost = post ? (
    <div className="flex flex-col w-3/4 justify-between mx-auto">
      <div className="text-3xl font-bold underline">{post.title}</div>
      <div className="text-xl font-light">{post.date}</div>
      {postContent ? <div dangerouslySetInnerHTML={{ __html: postContent}} /> : null }
      <a className="text-light text-xl underline" href={post.song_link}>Today's Song</a>
    </div>
  ) : null

  return (
    <>
      {blogHeader}
      {loading}
      {blogEntries}
      {blogPost}
    </>
  );
}

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById("root")
);
