//<input type="file" name="inputfile" id="inputfile">

document.getElementById('inputfile')
  .addEventListener('change', function () {
    let fr = new FileReader();
    fr.onload = function () {
      document.getElementById('output').textContent = fr.result;
    }
    fr.readAsText(this.files[0]);
})

function openCity(evt, cityName) {
  console.log("helooo")
  // Declare all variables
  var i, tablinks;

  // Get all elements with class="tablinks" and remove the class "active"
  tablinks = document.getElementsByClassName("tablinks");
  for (i = 0; i < tablinks.length; i++) {
    tablinks[i].className = tablinks[i].className.replace(" active", "");
  }

  // Show the current tab, and add an "active" class to the button that opened the tab
  document.getElementById(cityName).style.display = "block";
  evt.currentTarget.className += " active";
}
