import {IProxy, PaState} from "@/store/pastate";
import {CreateUserRequest, UserBasicInfo} from "@/store/models/user";
import {api_user_create} from "@/store/net/api_user_create";
import {Notify} from "@/util/notify";
import {api_user_login, UserLoginResponse} from "@/store/net/api_user_login";
import {api_verify_token} from "@/store/net/api_verify_token";
import {api_user_basic_info} from "@/store/net/api_user_basic_info";
import {Rules} from "@/store/models/_rules";
import {api_verify_code_get} from "@/store/net/api_verify_code_get";

export class LogProxy implements IProxy{
    check_logable_thenlog(name:string,pw:string):boolean{
        if(name.length==0){
            Notify.warn("登录信息填写","用户名为空")
            return false
        }
        if(pw.length==0){
            Notify.warn("登录信息填写","密码为空")
            return false
        }
        api_user_login(name,pw)
        return true
    }
    check_registable_thenregist(regivars:{
        email:string,
        username:string,
        pw:string,
        pw2:string,
        verify:string,
    }){
        //邮箱
        //用户名
        //密码
        if(!Rules.check_email(regivars.email)){
            Notify.warn("注册信息填写","邮箱格式错误")
            return
        }
        if(regivars.pw.length<8){
            Notify.warn("注册信息填写","密码长度至少8位")
            return
        }
        if(regivars.pw!=regivars.pw2){
            Notify.warn("注册信息填写","两次密码输入不同")
            return
        }
        //验证码
        if(!Rules.check_verify(regivars.verify)){
            Notify.warn("注册信息填写","验证码为数字")
            return;
        }
        api_user_create(new CreateUserRequest(
            regivars.email,
            regivars.pw,
            parseInt(regivars.verify),
            regivars.username)).then(res=>{
                if(res!=undefined){
                    this.log_succ(
                        res
                    )
                }
        })
    }
    show_log_gui(show:boolean,log_or_regi?:boolean){
        if(show&&log_or_regi!=undefined){
            this.state.log_gui_log_or_regi=log_or_regi;
            this.state.log_gui_show=show;
        }else{
            console.log("hide log gui")
            this.state.log_gui_show=false;
        }
    }
    switch_log_regi(){
        if(this.state.log_gui_show){
            this.state.log_gui_log_or_regi=!this.state.log_gui_log_or_regi
        }
    }
    get_logshowing(){
        console.log("get_logshowing",this.state.log_gui_show)
        return this.state.log_gui_show&&(this.state.logged_uid==-1)
    }
    get_log_or_regi(){
        return this.state.log_gui_log_or_regi
    }
    log_succ(res:UserLoginResponse){
        // this.state.logged_token;
        this.state.log_gui_show=false
        localStorage.logged_token=res.token
        localStorage.logged_uid=res.uid

        this.state.logged_token=res.token
        this.state.logged_uid=res.uid
        console.log("log succ",this.state.logged_uid)
        this.update_logged_userbasic()
    }
    log_out(){
        delete localStorage.logged_token
        delete localStorage.logged_uid

        this.state.logged_token=""
        this.state.logged_uid=-1
    }
    update_logged_userbasic(){
        if(this.state.logged_uid!=-1){
            api_user_basic_info(this.state.logged_uid).then((res)=>{
                if(res){
                    this.state.logged_basicinfo=res;
                }else{
                    Notify.warn("获取登入用户信息失败","")
                    window.setTimeout(()=>{
                        this.update_logged_userbasic()
                    },5000)
                }
            })
        }
    }
    token_verify(){
        let uid=localStorage.logged_uid
        if(uid!=undefined){
            uid=parseInt(uid)
        }
        const token=localStorage.logged_token
        function removeinvalid(){
            delete localStorage.logged_uid
            delete localStorage.logged_token
        }
        if(token!=undefined
            &&uid!=undefined
            &&token!=""){
            api_verify_token(
                uid,token
            ).then((res)=>{
                if(res=="invalid"){
                    removeinvalid()
                }else if(res=="expire"){
                    Notify.warn("登入信息已过期","")
                    removeinvalid()
                }else{
                    //覆盖token
                    localStorage.logged_token=res.newtoken
                    //验证成功后，
                    this.state.logged_token=res.newtoken
                    console.log("logged_uid",this.state.logged_uid,uid)
                    this.state.logged_uid=uid
                    this.update_logged_userbasic()
                    Notify.common("success","登入信息已更新","")
                }
            })
        }else{
            removeinvalid()
        }
    }
    get_verify_code(email:string){
        if(Rules.check_email(email)){
            api_verify_code_get(email).then(
                (ok)=>{

                }
            )
            return
        }
        Notify.warn("email 格式不正确","")
    }
    first_load(){
        //首次加载token，并检查token
        if(localStorage.logged_token
            &&localStorage.logged_token!=""){
            this.token_verify()
        }
    }
    get_logged_uid(){
        return this.state.logged_uid
    }
    get_logged_basic():UserBasicInfo|{}{
        return this.state.logged_basicinfo
    }
    get_logged_token(){
        return this.state.logged_token
    }
    constructor(private state:PaState) {
    }
}