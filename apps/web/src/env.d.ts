/// <reference types="astro/client" />

interface Submission {
  id: string
  user: string
  script: string
  comment: string
  wins: number
  issue_url: string
  issue_number: number
  created_at: string
  updated_at: string
  disqualifeid: boolean
  mmr: number
  matches_played: number
}

interface Match {
  id: string
  winner: User
  winner_submission: Submission
  loser: User
  loser_submission: Submission
  created_at: string
  updated_at: string
  p1_is_winner: number
  match_error?: string
}

interface User {
  id: string
  username: string
  created_at: string
  updated_at: string
}

interface Turn {
  id: string
  match_id: string
  turn: number
  board: string
  created_at: string
  updated_at: string
}

type MatchesResponse = {
  matches: string[]
}

interface MatchResponse {
  result: Match
  turns: Turn[]
}
