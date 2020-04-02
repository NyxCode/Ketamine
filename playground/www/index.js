import * as wasm from "playground";
import "jquery.json-viewer/json-viewer/jquery.json-viewer";
import "jquery.json-viewer/json-viewer/jquery.json-viewer.css";

$(document).delegate('#code', 'keydown', function (e) {
    var keyCode = e.keyCode || e.which;

    if (keyCode === 9) {
        e.preventDefault();
        let start = this.selectionStart;
        let end = this.selectionEnd;

        if (e.shiftKey) {
            let remove = $(this).val().substring(start - 4, start);
            if (remove === "    ") {
                $(this).val($(this).val().substring(0, start - 4) + $(this).val().substring(start))
                this.selectionStart = this.selectionEnd = start - 4;
            }
        } else {
            // set textarea value to: text before caret + tab + text after caret
            $(this).val($(this).val().substring(0, start)
                + "    "
                + $(this).val().substring(end));

            // put caret at right position again
            this.selectionStart =
                this.selectionEnd = start + 4;
        }

    }
});

$.fn.selectRange = function(start, end) {
    var e = document.getElementById($(this).attr('id')); // I don't know why... but $(this) don't want to work today :-/
    if (!e) return;
    else if (e.setSelectionRange) { e.focus(); e.setSelectionRange(start, end); } /* WebKit */
    else if (e.createTextRange) { var range = e.createTextRange(); range.collapse(true); range.moveEnd('character', end); range.moveStart('character', start); range.select(); } /* IE */
    else if (e.selectionStart) { e.selectionStart = start; e.selectionEnd = end; }
};

$(document).ready(function () {
    $('#tabs').on("click", ".item", function () {
        if (this.id === "output-tab") {
            run();
        } else if (this.id === "ast-tab") {
            parse();
        } else if (this.id === "tokens-tab") {
            lex();
        }
    });

    $("#clear").on("click", function () {
        $("#code").val("");
    });

    $("#run").on("click", run);
    $("#lex").on("click", lex);
    $("#parse").on("click", parse);

    run();
});

function run() {
    let code = $("#code").val();
    try {
        let runResult = wasm.run(code);
        displayOutput(runResult[0], runResult[1]);
    } catch (e) {
        displayError(e);
    }
    $("#output-tab")
        .addClass('active')
        .siblings('.item')
        .removeClass('active');
}


function parse() {
    let code = $("#code").val();
    try {
        let ast = wasm.parse(code);
        displayJson(ast);
    } catch (e) {
        displayError(e);
    }
    $("#ast-tab")
        .addClass('active')
        .siblings('.item')
        .removeClass('active');
}

function lex() {
    let code = $("#code").val();
    try {
        let tokens = wasm.lex(code);
        displayJson(tokens);
    } catch (e) {
        displayError(e)
    }
    $("#tokens-tab")
        .addClass('active')
        .siblings('.item')
        .removeClass('active');
}

function displayOutput(val, out) {
    let output = out.split(/\r\n|\r|\n/)
        .map(line => `<span class="mono-font">${$("<div>").text(line).html()}</span>`)
        .join("<br/>");
    let html = `
        ${output}
        <br/>
        <p>
            <i class="arrow right icon"></i> <span class="mono-font">${$("<div>").text(val).html()}</span>
        </p>
    `;
    $("#view").html(html);
}

function displayJson(json) {
    $("#view").html(`<pre id="json-renderer"></pre>`);
    $('#json-renderer').jsonViewer(json, {collapsed: true, rootCollapsable: false});
}

function displayError(error) {
    $("#code").selectRange(error.start, error.end);
    let lines = error.report.split(/\r\n|\r|\n/);
    let out = lines
        .map(line => `<span class="mono-font">${$("<div>").text(line).html()}</span>`).join("<br/>");
    $("#view").html(out);
}
