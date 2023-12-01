document.getElementById('inputfile')
            .addEventListener('change', function () {

                let fr = new FileReader();
                fr.onload = function () {
                    document.getElementById('output')
                        .textContent = fr.result;
                }

                fr.readAsText(this.files[0]);
            })
