import { Post, PostListing } from "./post";

const ROOT_URL = "http://localhost:8080/blogapi"; //"https://chrismiller.xyz/blogapi";

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
