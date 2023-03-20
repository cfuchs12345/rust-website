$(function() {
    initListeners();
});


function fadeAndLoad(htmlFile)  {
    let node = $("#content");

    if( node.hasClass("custom-notVisible")) {
        node.removeClass("custom-notVisible");
    }

    if( node.hasClass("custom-fadeIn")) {
        node.removeClass("custom-fadeIn");
        node.addClass("custom-fadeOut");

        setTimeout(function() {
            node.removeClass("custom-fadeOut");
            fadeAndLoad(htmlFile);
        }, 1000);
        return;
    }
    
    loadContent(node, htmlFile);
    node.addClass("custom-fadeIn");        
}

function loadContent(node, htmlFile) {
    let language = $('meta[name=language]').attr('content');

    
    if( language != undefined) {
        htmlFile +=  "?language=" + language;
    }
    
    node.load(htmlFile);        
}

function initListeners() {    
    $("#hrefGermanLanguage").on("click",function() {
        document.location.href = "?language=de";
    });

    $("#hrefEnglishLanguage").on("click",function() {
        document.location.href = "?language=en";
    });


    $("#hrefAboutMe").on("click",function() {
        fadeAndLoad( "aboutMe.html");
    });

    $("#hrefProjects").on("click",function()  {
        fadeAndLoad( "projects.html");
    });

    $("#hrefSkills").on("click",function()  {
        fadeAndLoad( "skills.html");
    });

    $("#hrefPrivateprojects").on("click",function()  {
        fadeAndLoad( "privateprojects.html");
    });

    $("#hrefContact").on("click",function(){
        fadeAndLoad( "contact.html");
    });

    $("#hrefLegalInfo").on("click",function(){
        fadeAndLoad( "legalInfo.html");
    });
    $("#hrefAboutPage").on("click",function(){
        fadeAndLoad( "aboutPage.html");
    });
}