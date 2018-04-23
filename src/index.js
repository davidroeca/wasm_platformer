import createImportObject from './js/createImportObject'
import gameLoop from './js/gameLoop'
import loadLib from './lib.rs'

loadLib({env: createImportObject()}).then(result => {
  const rustLib = result.instance.exports
  const background = document.getElementById('background')
  const game = rustLib.create_game()
  const resizeBackground = () => {
    background.width = window.innerWidth * 0.8
    background.height = window.innerHeight * 0.8
  }
  window.addEventListener('resize', resizeBackground)
  const handleKey = (key, isDown) => {
    switch (key) {
      case "ArrowUp":
        rustLib.toggle_move_up(game, isDown)
        break
      case "ArrowDown":
        rustLib.toggle_move_down(game, isDown)
        break
      case "ArrowRight":
        rustLib.toggle_move_right(game, isDown)
        break
      case "ArrowLeft":
        rustLib.toggle_move_left(game, isDown)
        break
      //case " ":
        //rustLib.toggle_shoot(isPress)
        //break
    }
  }
  document.addEventListener('keydown', event => handleKey(event.key, true))
  document.addEventListener('keyup', event => handleKey(event.key, false))
  gameLoop(
    game,
    rustLib.update,
    rustLib.render
  )
  resizeBackground()
})
