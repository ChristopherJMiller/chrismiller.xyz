import React from "react";
import { PostListing } from "./post";

interface BlogEntryProps {
  post: PostListing,
}

export const BlogEntry = ({ post }: BlogEntryProps) => (
  <div className="flex flex-col mb-10">
    <a href={`/blog.html?post=${post.path}`} className="text-xl font-semibold underline">{post.name}</a>
    <div>{post.description}</div>
  </div>
)
