import {history} from "umi";
function getUrlParams (query:string) {
    let urlParam:any = {};
    if(query){
        const paramArr = query.split('&');
        for(let i=0 ; i<paramArr.length ; i++ ){

            if(i == 0) paramArr[i] = paramArr[i].substr(1,paramArr[i].length);
            let arr = paramArr[i].split('=');
            urlParam[arr[0]] = arr[1];
        }
    }
    return urlParam;
}
function UrlParams2Suffix(params:any){
    if(Object.keys(params).length==0){
        return ""
    }
    let make="?"
    let first=true
    for (const key in params){
        if(!first){
            make+='&'
        }
        first=false;
        make+=key+'='+params[key]
    }
    return make
}
export namespace RouteControl{
    export function open_user(uid:number){
        if(window.location.pathname=="/user/"+uid||
            window.location.pathname=="/user/"+uid+'/'
        ){
            return
        }
        history.push("/user/"+uid)
    }
    export function get_article_mode():"create"|"edit"|"view"{
        const params=getUrlParams(window.location.search)
        if(params.mode=="create"){
            return "create"
        }
        if(params.mode=="edit"){
            return "edit"
        }
        return "view"
    }
    export function push_create_article(){
        history.push("/article/_?mode=create")
    }
}