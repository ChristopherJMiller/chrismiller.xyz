import React from 'react';

interface TagListProps {
  tags: string[],
  activeTag: string | null,
  onTagSelected: (tag: string) => void
}

export default ({ tags, activeTag, onTagSelected }: TagListProps) => {

  const tagPills = tags.map((tag) => {
    const classes = tag === activeTag ?
      "bg-cyan-600 cursor-pointer text-white rounded m-1 p-1 transition-all ease-in-out delay-50 duration-200 hover:-translate-y-1 hover:scale-105"
      : "bg-cyan-500 cursor-pointer text-white rounded m-1 p-1 transition-all ease-in-out delay-50 duration-200 hover:-translate-y-1 hover:scale-105"
    
    return <div className={classes} onClick={() => onTagSelected(tag)}>{tag}</div>
  });

  return (
    <div className="flex flex-nowrap w-100 overflow-x-scroll py-2 my-1">
      {tagPills}
    </div>
  )
}
