# rs-sfomuseum-bcbp

SFO Museum-specific Rust-based WebAssembly (WASM) binary wrapping the `martinmroz/iata_bcbp` crate.

## Important

This is both a work-in-progress and a learning exercise to get familiar with Rust.

It does _not_ return all of the data produced by the `martinmroz/iata_bcbp` crate but rather a list of truncated data for each leg of a flight.

## Example

```
$> make wasm
wasm-pack build --target web
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
   Compiling rs-sfomuseum-bcbp v0.1.0 (/usr/local/sfomuseum/rs-bcbp-wasm)
    Finished release [optimized] target(s) in 0.42s
[INFO]: ⬇️  Installing wasm-bindgen...
[INFO]: found wasm-opt at "/usr/local/bin/wasm-opt"
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: ✨   Done in 1.42s
[INFO]: 📦   Your wasm pkg is ready to publish at /usr/local/sfomuseum/rs-bcbp-wasm/pkg.
```

And then load the WASM binary in a webpage like this:

```
<html>
    <head>
	<title>barcode</title>
    </head>
    <body>
	<form>
	    <input type="text" id="barcode" disabled="disabled" />
	    <button type="submit" id="submit" disabled="disabled">Scan</button>
	</form>
	<ul id="legs">
	</ul>
    </body>
    <script type="module">

     import init, { parse } from './rs_sfomuseum_bcbp.js';

     async function main() {
            await init();

	 var barcode_el = document.getElementById("barcode");	    
	 var submit_el = document.getElementById("submit");
	 var legs_el = document.getElementById("legs");	 

	 barcode_el.removeAttribute("disabled");	    
	 submit_el.removeAttribute("disabled");
	 
	 submit_el.onclick = function(e){

	     legs_el.innerHTML = "";

	     try {
		 
		 let raw = barcode.value;
		 let rsp = parse(raw);
		 let count = rsp.length;
	     
		 for (var i=0; i < count; i++){
		     
		     var item = document.createElement("li");
		     item.appendChild(document.createTextNode(rsp[i].string()));
		 }
		 
		 legs_el.appendChild(item);
		 
	     } catch(err){

		 var item = document.createElement("li");
		 item.appendChild(document.createTextNode(err));
		 legs_el.appendChild(item);		 
		 console.log(err);
	     };
	     
	     return false;
	 };
	 
     }
     
     main();

    </script>
</html>
```

And then if you load that web page in a browser and parse the string `M1DESMARAIS/LUC       EABC123 YULFRAAC 0834 326J001A0025 100` you should see something like this:

![](docs/images/rs-sfomuseum-bcbp.png)

## See also

* https://github.com/martinmroz/iata_bcbp