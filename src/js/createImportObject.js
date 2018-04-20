// Define the foreign interface for rust to interact with

// Creates the player, among other resources
const getResources = () => {
  const player = document.createElement('canvas')
  player.height = 20
  player.width = 20
  const ctx = player.getContext('2d')
  ctx.fillStyle = 'blue'
  ctx.moveTo(0, 20)
  ctx.lineTo(10, 0)
  ctx.lineTo(20, 20)
  ctx.lineTo(14, 15)
  ctx.lineTo(6, 15)
  ctx.lineTo(0, 20)
  ctx.fill()
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
