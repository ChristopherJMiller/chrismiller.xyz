import React from 'react';

export interface PostListing {
  /// TODO change this field to title
  name: string,
  description: string,
  path: string,
}

export interface Post {
  title: string,
  date: string,
  cover_photo: string,
  cover_caption: string,
  tags: string[],
  song_link: string,
  content: string,
}

export const processPostContent = (content: string): JSX.Element[] => {
  const lines = content.split("\n");
  return lines.map((line) => {
    if (line.startsWith("##")) {
      return <div className="mt-3 text-xl text-semibold font-medium">{line.substring(2)}</div>;
    } else if (line.startsWith("#")) {
      return <div className="mt-3 text-2xl text-semibold font-medium">{line.substring(1)}</div>;
    } else if (line.startsWith(">")) {
      return <blockquote className="my-6 p-4 border-l-4 bg-neutral-100 text-neutral-600 border-neutral-500 quote">{line.substring(1)}</blockquote>
    }
    return <p className="my-4 text-serif">{line}</p>;
  });
}
