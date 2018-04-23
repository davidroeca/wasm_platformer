import createImportObject from './js/createImportObject'
import gameLoop from './js/gameLoop'
import loadLib from './lib.rs'

loadLib({env: createImportObject()}).then(result => {
  const rustLib = result.instance.exports
  const background = document.getElementById('background')
  const resizeBackground = () => {
    background.width = window.innerWidth * 0.8
    background.height = window.innerHeight * 0.8
  }
  window.addEventListener('resize', resizeBackground)
  const handleKey = (key, isPress) => {
    switch (key) {
      case "ArrowUp":
        rustLib.toggle_move_up(isPress)
        break
      case "ArrowDown":
        rustLib.toggle_move_down(isPress)
        break
      case "ArrowRight":
        rustLib.toggle_move_right(isPress)
        break
      case "ArrowLeft":
        rustLib.toggle_move_left(isPress)
        break
      case " ":
        rustLib.toggle_shoot(isPress)
        break
    }
  }
  document.addEventListener('keydown', event => handleKey(event.key, 1))
  document.addEventListener('keyup', event => handleKey(event.key, 0))
  gameLoop(
    rustLib.update,
    rustLib.render
  )
  resizeBackground()
})
