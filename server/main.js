const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d');
var width =  window.innerWidth;
var height = window.innerHeight;
var balls = [];
var a=[];
var evilCircle;
var getForce = document.querySelector(".force");
var gravButton = document.querySelector(".gruvity")
var score = document.querySelector(".score");
var gameBtn = document.querySelector(".game");
var gravityFlag = false;
var gameFlag = false;
canvas.width = 2000;
canvas.height = 1000;

class Ball {
  constructor (x, y, velX, velY, color, size) {
    this.x = x;
    this.y = y;
    this.velX = velX;
    this.velY = velY;
    this.color = color;
    this.size = size;
  }
  draw() {
    ctx.beginPath();
    ctx.fillStyle = this.color;
    ctx.arc(this.x, this.y, this.size, 0, 2 * Math.PI);
    ctx.fill();
  }
  update() {
    if ((this.x + this.size) >= width) {
      this.velX = -(this.velX);
    }
  
    if ((this.x - this.size) <= 0) {
      this.velX = -(this.velX);
    }
  
    if ((this.y + this.size) >= height) {
      this.velY = -(this.velY);
    }
  
    if ((this.y - this.size) <= 0) {
      this.velY = -(this.velY);
    }
    
    this.x += this.velX;
    this.y += this.velY;
  }
  checkColision () {
    for (var ball of balls) {
      if (!(ball === this)) {
        var dx = this.x - ball.x;
        var dy = this.y - ball.y;
        var distance = Math.sqrt(dx * dx + dy *dy);
        if (distance < (this.size + ball.size)) {
          this.color = ball.color = randomRGB();
          
          // ball.color = randomRGB();
        }
      }
    }
  }
  force () {
    this.velY += random(-7, -12);
  }
}

class EvilCircle extends Ball {
  constructor (x, y ,velX, velY, color, size) {
    super(x, y ,velX, velY, color, size);
  }
  draw() {
    ctx.beginPath();
    ctx.strokeStyle = this.color;
    ctx.lineWidth = 2;
    ctx.arc(this.x, this.y, this.size, 0, 2 * Math.PI);
    ctx.stroke();
  }
  checkColision() {
    a = [];
    var x = this.x;
    var y = this.y;
    var size = this.size;
    balls.forEach(function(item) {
      var dx = x - item.x;
      var dy = y - item.y;
      var distance = Math.sqrt(dx*dx + dy*dy);
      if (distance < (size  + item.size)) {
        
      } else {
        a.push(item);
      }

    })
    balls = a;
   
  }
  move() {
    window.addEventListener("keydown", (e) => {
      switch(e.key) {
        case("a"):
          this.x -= this.velX;
          break;
        case("d"):
          this.x += this.velX;
          break;
        case("w"):
          this.y -= this.velY;
          break;
        case("s"):
          this.y += this.velY;
          break;
      };
    });
  }
  
}

function startGame() {
  evilCircle = new EvilCircle(200, 200 ,20, 20, "rgb(255, 255, 255)", 20);
  evilCircle.move();
  gameFlag = true;
  gameBtn.style.display = "none";

}


genBalls(20);
function loop() {
  if (window.innerWidth != width) {
    width = window.innerWidth;
    height = window.innerHeight;
  }
  ctx.fillStyle = "rgba(0, 0, 0, 0.2)";
  ctx.fillRect(0, 0, width, height);
  if (gameFlag) {
    evilCircle.draw();
    evilCircle.checkColision();
  }
  for (const ball of balls) {
    ball.draw();
    ball.update();
    ball.checkColision();
    if (gravityFlag) {
      ball.velY +=0.15;
      
      if (ball.velY > 0) {
        ball.velY -=0.1;
      }
    }
  }
  score.textContent = "Circle left: " + balls.length;
  requestAnimationFrame(loop);
}






// function to generate random number

function random(min, max) {
  const num = Math.floor(Math.random() * (max - min + 1)) + min;
  return num;
}

// function to generate random color

function randomRGB() {
  return `rgb(${random(0, 255)},${random(0, 255)},${random(0, 255)})`;
}

function genBalls(count) {
  while (balls.length < count) {
    const size = random(10, width * 0.015);
    const ball = new Ball(
      random(0 + size, width - size),
      random(0 + size, height - size),
      random(-7, 7),
      random(-7, 7),
      randomRGB(),
      size
    );
    balls.push(ball);
  }

}

function getF() {
  for(var ball of balls) {
    ball.force();
  }
}

function onGravity() {
  if(gravityFlag==true) {
    gravityFlag=false;
    gravButton.textContent = "On gravity";
    getForce.style.display = "none";
  } else {
    gravityFlag=true;
    gravButton.textContent = "Off gravity";
    getForce.style.display = "block";
  }
}

getForce.addEventListener("click", getF);
gravButton.addEventListener("click", onGravity);
gameBtn.addEventListener("click", startGame)
loop();