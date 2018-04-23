// Define utility for getting the game's timestamp
const getTimeStamp = () => (
  window.performance && window.performance.now ?
    window.performance.now() :
    new Date().getTime()
)

// Deifne the loop itself
const gameLoop = (game, update, render, fps = 60) => {
  const step = 1 / fps
  let last = null

  const frame = () => {
    if (!last) {
      last = getTimeStamp()
      requestAnimationFrame(frame)
      return
    }
    const now = getTimeStamp()
    let dt = Math.min(1, (now - last) / 1000)
    while (dt >= step) {
      update(game, dt)
      dt -= step
    }
    render(game, dt)
    last = now
    requestAnimationFrame(frame)
  }

  requestAnimationFrame(frame)
}

export default gameLoop
