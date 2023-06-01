import { useEffect, useRef, useState } from "react"
import { Tile } from "./tile"

const UPDATE_INTERVAL = 500
export function Board({ winner, board }: { winner: number; board: string[] }) {
  const [turnIndex, setTurnIndex] = useState(0)

  const timer = useRef<number>()

  useEffect(() => {
    timer.current = setTimeout(onTimerUpdate, UPDATE_INTERVAL)

    return () => clearTimeout(timer.current)
  }, [turnIndex])

  function onTimerUpdate() {
    setTurnIndex((turnIndex) => (turnIndex + 1) % (board.length + 1))
    timer.current = setTimeout(onTimerUpdate, UPDATE_INTERVAL)
  }

  return (
    <div
      className={`grid-rows-9 grid aspect-square w-2/3 max-w-md grid-cols-9 items-center justify-center ${
        winner === 0 ? "bg-green-500" : "bg-red-500"
      }`}
    >
      {turnIndex !== board.length && (
        <>
          {board[turnIndex]?.split("").map((cell, index) => (
            <Tile key={index} tileType={cell} />
          ))}
        </>
      )}
    </div>
  )
}
