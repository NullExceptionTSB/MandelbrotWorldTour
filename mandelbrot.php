<?php
function Re($c) { return $c[0]; }
function Im($c) { return $c[1]; }

function sqr_complex($c) {
	$real = Re($c) * Re($c) - Im($c) * Im($c);
	$imaginary = 2*Re($c)*Im($c);
	return array($real, $imaginary);
}

header("Content-Type: image/png");
$im = imagecreate(1024, 1024);
$colors = array(20);

for ($i = 0; $i < 20; $i++) {
	$fi = (float)$i;
	$colors[$i] = imagecolorallocate($im, 0, (int) ((($fi*$fi)/2+1)*255)/800, (int)((($fi*$fi)/2+1)*255)/200);
}
for ($x = 0; $x < 1024; $x++) {
	for ($y = 0; $y < 1024; $y++){
		$c = array((((float)$x)/1024)*4-2.5, (((float)$y)/1024)*4-2);
		
		//var_dump($c);
		$z = array((float)0.0, (float)0.0);
		imagesetpixel($im, $x, $y, 0);
		for ($i = 0; $i < 20; $i++) {
			$z = sqr_complex($z);
			$z[0] = Re($z) + Re($c);
			$z[1] = Im($z) + Im($c);
			
			if (abs(Re($z)) > 2 || abs(Im($z)) > 2) {
				imagesetpixel($im, $x, $y, $i);				
				break;
			}
			if ($i > 0 && Re($z) == 0 && Im($z) == 0) break;
		}
		
	}
}

imagebmp($im);
imagedestroy($im);
?>
