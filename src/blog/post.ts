
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
  small_cover_photo: string,
  cover_caption: string,
  tags: string[],
  song_link: string,
  content: string,
}
