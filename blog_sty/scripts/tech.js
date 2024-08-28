// Select the tooltip by class
var tooltip = document.querySelectorAll('.tooltip');

// Move the tooltip with the mouse
document.addEventListener('mousemove', fn, false);

// Event listener
function fn(e) {
    for (var i=tooltip.length; i--;) {
        tooltip[i].style.left = e.pageX + 'px';
        tooltip[i].style.top = e.pageY + 'px';
    }
}