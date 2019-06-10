import { Ball, Paddle } from "pong";

let ball
let playerPaddle = Paddle.new_player()
let computerPaddle = Paddle.new_computer()
const height = 750
const width = 1250

const canvas = document.getElementById("pong-canvas")
canvas.height = height
canvas.width = width
const ctx = canvas.getContext('2d')

const clearRect = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height)
}

const getRandomInt = (max) => Math.floor(Math.random() * Math.floor(max))

const generateBall = () => {
  const x = getRandomInt(1250)
  const y = getRandomInt(750)
  // ball = Ball.new(x, 375, 10, 0)
  ball = Ball.new(625, y, 5, 5)
}


generateBall()


const animate = window.requestAnimationFrame ||
                window.webkitRequestAnimationFrame ||
                window.mozRequestAnimationFrame ||
                function(callback) { window.setTimeout(callback, 1000/60) }

const renderPaddles = () => {
  ctx.fillRect(playerPaddle.get_x(), playerPaddle.get_y(), playerPaddle.get_width(), playerPaddle.get_height())
  ctx.fillRect(computerPaddle.get_x(), computerPaddle.get_y(), computerPaddle.get_width(), computerPaddle.get_height())
}


document.addEventListener('keydown', (event) => {
  const key = event.key
  if(key === 's'){
    playerPaddle.move_up(20)
  } else if(key === 'w'){
    playerPaddle.move_down(-20)
  }
})

const renderBall = () => {
  if(!ball.get_in_play()){
    generateBall()
  }
  ctx.beginPath()
  ctx.arc(ball.get_x(), ball.get_y(), ball.get_radius(), 2 * Math.PI, false)
  ctx.fill()
}

var step = () => {
  update()
  render()
  animate(step)
}

const update = () => {
  ball.move_ball(playerPaddle, computerPaddle)
  computerPaddle.computer_ai(ball)
};

var render = () => {
  clearRect()
  renderBall()
  renderPaddles()
};
                

window.onload = () => {
  document.body.appendChild(canvas)
  animate(step);
}
