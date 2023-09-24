// Wasming
// compile: GOOS=js GOARCH=wasm go build -o main.wasm ./main.go
package main

import (
	"fmt"
	"math"
	"math/rand"
	"syscall/js"
)

var (
	width      float64
	height     float64
	mousePos   [2]float64
	ctx        js.Value
	lineDistSq float64 = 100 * 100
)

type Ball struct {
	x     float64
	y     float64
	velX  float64
	velY  float64
	color string
	size  float64
}

func NewBall(
	x float64,
	y float64,
	velX float64,
	velY float64,
	color string,
	size float64,
) *Ball {
	b := Ball{x, y, velX, velY, color, size}
	return &b
}

func main() {
	doc := js.Global().Get("document")
	canvas := doc.Call("getElementById", "canvas")
	width = doc.Get("body").Get("clientWidth").Float()
	height = doc.Get("body").Get("clientHeight").Float()
	canvas.Set("width", width)
	canvas.Set("height", height)
	ctx = canvas.Call("getContext", "2d")

	balls := []*Ball{}

	for len(balls) < 25 {
		size := rand.Intn(10) + 10
		x := rand.Float64() * width
		y := rand.Float64() * height
		velX := float64(rand.Intn(20)) -20
		velY := float64(rand.Intn(20)) -20
		r := rand.Intn(255)
		g := rand.Intn(255)
		b := rand.Intn(255)
		color := fmt.Sprintf("rgb(%d, %d, %d)", r, g, b)
		ball := NewBall(x, y, velX, velY, color, float64(size))
		balls = append(balls, ball)
	}
  var (
		renderFrame js.Func
	)

	renderFrame = js.FuncOf(func(this js.Value, args []js.Value) interface{} {
    loop(balls)
		js.Global().Call("requestAnimationFrame", renderFrame)
		return nil
	})
	js.Global().Call("requestAnimationFrame", renderFrame)
	done := make(chan struct{}, 0)
	<-done
}

func loop(b []*Ball) {
	ctx.Set("fillStyle", "rgba(0,0,0,0.25)")
	ctx.Call("fillRect", 0, 0, width, height)
	for i := 0; i < len(b); i++ {
		b[i].draw()
		b[i].update()
	}
}

func (b *Ball) draw() {
	ctx.Call("beginPath")
	ctx.Set("fillStyle", fmt.Sprintf(b.color))
	ctx.Call("arc", b.x, b.y, b.size, 0, 2*math.Pi)
	ctx.Call("fill")
}

func (b *Ball) update() {
	if (b.x + b.size) >= width {
		b.velX = -(b.velX)
	}

	if (b.x - b.size) <= 0 {
		b.velX = -(b.velX)
	}

	if (b.y + b.size) >= height {
		b.velY = -(b.velY)
	}

	if (b.y - b.size) <= 0 {
		b.velY = -(b.velY)
	}

	b.x += b.velX
	b.y += b.velY
}
