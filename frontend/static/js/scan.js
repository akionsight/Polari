const video       = document.getElementById('video');
    const canvas      = document.getElementById('canvas');
    const ctx         = canvas.getContext('2d', { willReadFrequently: true });
    const statusEl    = document.getElementById('status');
    const overlay     = document.getElementById('successOverlay');
    const manualBtn   = document.getElementById('manualBtn');
    const manualInput = document.getElementById('manualInput');

    let scanning = true;
    let barcodeDetector = null;
    let scanMethod = 'unknown';


    // ── Choose scan method ──
    if (typeof BarcodeDetector !== 'undefined') {
        BarcodeDetector.getSupportedFormats().then(formats => {
            if (formats.includes('qr_code')) {
                barcodeDetector = new BarcodeDetector({ formats: ['qr_code'] });
                scanMethod = 'BarcodeDetector';
            } else {
                scanMethod = 'canvas-manual';
            }
            startCamera();
        });
    } else {
        // Load jsQR dynamically from multiple CDN fallbacks
        loadJsQR();
    }

    function loadJsQR() {
        const cdns = [
            'https://cdn.jsdelivr.net/npm/jsqr@1.4.0/dist/jsQR.js',
            'https://unpkg.com/jsqr@1.4.0/dist/jsQR.js',
            'https://cdnjs.cloudflare.com/ajax/libs/jsQR/1.4.0/jsQR.min.js'
        ];
        let idx = 0;
        function tryNext() {
            if (idx >= cdns.length) {
                statusEl.textContent = 'QR library failed to load. Use manual entry below.';
                startCamera(); // still start camera so user sees it
                return;
            }
            const url = cdns[idx++];
            const s = document.createElement('script');
            s.src = url;
            s.onload = () => {
                if (typeof jsQR !== 'undefined') {
                    scanMethod = 'jsQR';
                    startCamera();
                } else {
                    tryNext();
                }
            };
            s.onerror = () => { tryNext(); };
            document.head.appendChild(s);
        }
        tryNext();
    }

    async function startCamera() {
        try {
            const stream = await navigator.mediaDevices.getUserMedia({
                video: {
                    facingMode: { ideal: 'environment' },
                    width:  { ideal: 1280 },
                    height: { ideal: 720 }
                }
            });
            video.srcObject = stream;

            video.addEventListener('loadedmetadata', () => {
            });

            video.addEventListener('playing', () => {
                statusEl.textContent = 'Scanning…';
                requestAnimationFrame(tick);
            }, { once: true });

            await video.play().catch(() => {});

        } catch (err) {
            statusEl.textContent = 'Camera denied – use manual entry below.';
            statusEl.style.color = '#ff6b6b';
        }
    }

    async function tick() {
        if (!scanning) return;
        if (video.readyState < 2 || video.videoWidth === 0) {
            requestAnimationFrame(tick);
            return;
        }

        if (canvas.width !== video.videoWidth) {
            canvas.width = video.videoWidth;
            canvas.height = video.videoHeight;
        }


        let result = null;

        if (barcodeDetector) {
            try {
                const codes = await barcodeDetector.detect(video);
                if (codes.length > 0) result = codes[0].rawValue;
            } catch(e) {
            }
        }

        if (!result && scanMethod === 'jsQR' && typeof jsQR !== 'undefined') {
            ctx.drawImage(video, 0, 0, canvas.width, canvas.height);
            try {
                const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
                const code = jsQR(imageData.data, imageData.width, imageData.height, {
                    inversionAttempts: 'attemptBoth'
                });
                if (code && code.data) result = code.data;
            } catch(e) {
            }
        }

        if (result) {
            handleResult(result);
            return;
        }

        requestAnimationFrame(tick);
    }

    function handleResult(raw) {
        scanning = false;
        const uuidRegex = /[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/i;
        const match = raw.match(uuidRegex);
        const id = match ? match[0] : raw.trim();

        statusEl.textContent = 'Found: ' + id;
        statusEl.style.color = 'var(--button-yellow)';
        overlay.classList.add('show');

        setTimeout(() => {
            window.location.href = 'track.html?id=' + encodeURIComponent(id);
        }, 1500);
    }

    manualBtn.addEventListener('click', () => {
        const val = manualInput.value.trim();
        if (!val) return;
        handleResult(val);
    });

    manualInput.addEventListener('keydown', e => {
        if (e.key === 'Enter') manualBtn.click();
    });