// Define the foreign interface for rust to interact with

// Creates the player, among other resources
const getResources = () => {
  const player = document.createElement('canvas')
  player.height = 10
  player.width = 10
  const playerContext = player.getContext('2d')
  playerContext.fillStyle = 'blue'
  playerContext.fillRect(0, 0, 10, 10)
  return {
    player
  }
}

// Define and export the foreign functions to be used by rust
const createImportObject = () => {
  const background = document.getElementById('background')
  const { player } = getResources()
  const ctx = background.getContext('2d')
  const clear_screen = () => {
    ctx.fillStyle = 'white'
    ctx.fillRect(0, 0, background.width, background.height)
  }
  const draw_player = (x, y) => {
    ctx.drawImage(player, x, y)
  }
  return {
    clear_screen,
    draw_player,
  }
}

export default createImportObject
