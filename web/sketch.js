let angle = 0.3;
let frameIter = 0;
const maxiterations = 200;
const frameLayer = 2;
const colorsRed = [];
const colorsGreen = [];
const colorsBlue = [];

function setup() {
    pixelDensity(1);
    createCanvas(window.innerWidth, window.innerHeight);
    background(255);
    colorMode(HSB, 1);
    for (let n = 0; n < maxiterations; n++) {
        let hu = sqrt(n / maxiterations);
        let col = color(hu, 255, 150);
        colorsRed[n] = red(col);
        colorsGreen[n] = green(col);
        colorsBlue[n] = blue(col);
    }
    loadPixels();
}

function draw() {
    let ca = cos(angle * 3.213);
    let cb = sin(angle);
    angle += 0.0002;
    frameIter++;
    let w = 5;
    let h = (w * height) / width;
    let xmin = -w / 8;
    let ymin = -h / 8;
    let xmax = xmin + w;
    let ymax = ymin + h;
    let dx = (xmax - xmin) / width / 4;
    let dy = (ymax - ymin) / height / 4;
    let y = ymin;
    for (let j = frameIter % frameLayer; j < height / frameLayer; j++) {
        let x = xmin;
        for (let i = 0; i < width; i++) {
            let a = x;
            let b = y;
            let n = 0;
            while (n < maxiterations) {
                let aa = a * a;
                let bb = b * b;
                if (aa + bb > 4.0) {
                    break;
                }
                let twoab = 2.0 * a * b;
                a = aa - bb + ca;
                b = twoab + cb;
                n++;
            }
            let pix = (i + ((j * frameLayer - frameIter % frameLayer) * width)) * 4;
            if (n == maxiterations) {
                pixels[pix + 0] = 0;
                pixels[pix + 1] = 0;
                pixels[pix + 2] = 0;
            } else {
                pixels[pix + 0] = colorsRed[n];
                pixels[pix + 1] = colorsRed[n];
                pixels[pix + 2] = colorsRed[n];
            }
            x += dx;
        }
        y += dy * frameLayer;
    }
    updatePixels();
    colorMode(RGB, 255);
    fill(153, 140, 54 / 2, 200);
    rect(0, 0, width, height);
}