Bitmap mandelbrot() {
	Bitmap bmp = new Bitmap(1024, 1024);
	
	for (int x = 0; x < 1024; x++) for (int y = 0; y < 1024; y++) {
		Complex c = new Complex(4.0f * x / 1024 - 2.5f, 4.0f * y / 1024 - 2.0f), z = 0;
		bmp.SetPixel(x, y, Color.Black);
		for (int i = 0; i < 20; i++) {
			z = Complex.Pow(z, 2) + c;
			if (z.Real > 2 || z.Imaginary > 2) {
				bmp.SetPixel(x, y, Color.FromArgb(255, 0, (((i * i) / 2 + 1) * 255) / 800, (((i*i)/2+1) * 255) / 200));
				break;
			}
			if (i > 0 && z.Real == 0 && z.Imaginary == 0) break;
		}
	}
	return bmp;
}