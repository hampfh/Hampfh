function getTileColor(tileType: number) {
  switch (tileType) {
    case 0:
      return "bg-zinc-700"
    case 1:
      return "bg-red-500"
    case 2:
      return "bg-green-500"
    case 3:
      return "bg-black"
    default:
      return "bg-purple-500"
  }
}

export function Tile({ tileType }: { tileType: string }) {
  const bgColor = getTileColor(parseInt(tileType))
  return <div className={` flex h-full w-full flex-grow ${bgColor}`}></div>
}
