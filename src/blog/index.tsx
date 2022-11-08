import React, { useEffect, useState } from "react";
import ReactDOM from "react-dom";
import { getPost, getPostContentImage, getPostList, getPostSmallContentImage } from "./api";
import { Post, PostListing } from "./post";

import {unified} from 'unified'
import remarkParse from 'remark-parse'
import remarkGfm from 'remark-gfm'
import remarkRehype from 'remark-rehype'
import rehypeStringify from 'rehype-stringify'
import { LazyLoadImage } from "react-lazy-load-image-component";

import 'react-lazy-load-image-component/src/effects/blur.css';
import { BlogEntry } from "./entry";

const App = () => {
  const [postListings, setPostListings] = useState<PostListing[] | null>(null);
  const [post, setPost] = useState<Post | null>(null);
  const [postContent, setPostContent] = useState<string>();
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);
  const postParam: string | null = urlParams.get("post");


  useEffect(() => {
    const processP = async () => {
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
        .replaceAll("<h3>", `<h3 class="mt-3 text-lg text-semibold font-medium">`)
        
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

  const loading = postListings || post ? null : <div className="text-xl text-center my-5">Loading...</div>;

  const blogHeader = postListings ? <div className="text-3xl font-bold underline mb-3">Blog</div> : null;
  const blogEntries = postListings ? postListings.map((post) => <BlogEntry post={post} />) : null;  

  const blogPost = post ? (
    <div className="my-6 flex flex-col w-3/4 justify-between mx-auto">
      <LazyLoadImage
        alt={post.cover_caption}
        placeholderSrc={getPostSmallContentImage(post)}
        effect="blur"
        src={getPostContentImage(post)} />
      <span className="my-2 text-lg font-light">{post.cover_caption}</span>
      <div className="text-3xl font-bold underline">{post.title}</div>
      <div className="text-xl font-light">{post.date}</div>
      {postContent ? <div dangerouslySetInnerHTML={{ __html: postContent}} /> : null }
      <p className="text-light text-2xl underline my-3">Today's Song</p>
      <iframe style={{ borderRadius: "12px"}} src={post.song_link.replace("track", "embed/track").replace("album", "embed/album")} width="100%" height="80" frameBorder="0" allow="clipboard-write; encrypted-media"></iframe>
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
