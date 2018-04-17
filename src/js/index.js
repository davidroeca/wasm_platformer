const BACKGROUND_ID = 'background'

// Creates the game background
const createBackground = () => {
  const oldBackground = document.getElementById(BACKGROUND_ID)
  if (oldBackground) {
    return oldBackground
  }
  const background = document.createElement('canvas')
  background.width = '100%'
  background.height = '100%'
  background.id = BACKGROUND_ID
  document.body.appendChild(background)
  return document.getElementById(BACKGROUND_ID)
}

// Creates the player, among other resources
const getResources = () => {
  const player = document.createElement('canvas')
  player.height = 10
  player.width = 10
  const playerContext = player.getContext('2d')
  playerContext.fillStyle = 'blue'
  playerContext.fillRect(10, 10, 20, 20)
  return {
    player
  }
}

export const rustFFI = () => {
  const background = createBackground()
  const ctx = background.getContext('2d')
  const clear_screen = () => {
    ctx.fillStyle = 'white'
    ctx.fillRect(0, 0, background.width, background.height)
  }

  const draw_player = (x, y) => {
    ctx.translate(x, y)
  }
}
