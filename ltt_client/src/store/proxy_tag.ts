import {IProxy, PaState} from "@/store/pastate";
import {CreateUserRequest, UserBasicInfo} from "@/store/models/user";
import {api_user_create} from "@/store/net/api_user_create";
import {Notify} from "@/util/notify";
import {api_user_login, UserLoginResponse} from "@/store/net/api_user_login";
import {api_verify_token} from "@/store/net/api_verify_token";
import {api_user_basic_info} from "@/store/net/api_user_basic_info";
import {Rules} from "@/store/models/_rules";
import {api_verify_code_get} from "@/store/net/api_verify_code_get";
import {api_tags_fetch} from "@/store/net/api_tags_fetch";

export class TagProxy implements IProxy{

    constructor(private state:PaState) {
    }
    gettagset(){
        return this.state.tags.tagsets
    }
    first_load(): void {
        api_tags_fetch().then((res)=>{
            console.log("api_tags_fetch",res)
            if(res){
                this.state.tags.tagsets=res
            }
        })
    }
}