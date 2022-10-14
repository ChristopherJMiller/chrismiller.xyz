import { Post, PostListing } from "./post";

const ROOT_URL = "https://chrismiller.xyz/blogapi";

export const getPostList = async (): Promise<PostListing[]> => {
  const response = await fetch(`${ROOT_URL}/index.json`);
  const data: PostListing[] = await response.json();
  return data;
}

export const getPost = async (post: string): Promise<Post> => {
  const response = await fetch(`${ROOT_URL}/posts/${post}.json`);
  const data: Post = await response.json();
  return data;
}

export const getPostContentImage = (post: Post): string => `${ROOT_URL}/images/${post.cover_photo}`;
export const getPostSmallContentImage = (post: Post): string => `${ROOT_URL}/images/${post.small_cover_photo}`;
