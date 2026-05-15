package com.linkyoyo.tobacco.result;

import org.springframework.lang.Nullable;

import java.util.HashMap;
import java.util.Map;

public class R extends HashMap<String, Object> {
	
	private static final long serialVersionUID = 1L;

	public R(CodeMsg codeMsg) {
		put("code", codeMsg.getCode());
		put("msg", codeMsg.getMsg());
	}
	
	public R() {
		this(CodeMsg.SUCCESS);
	}
	
	public static R error() {
		return new R(CodeMsg.SERVER_ERROR);
	}

	public static  R tipInfo(CodeMsg codeMsg){return  new R(codeMsg);}

	public static R alertInfo(CodeMsg codeMsg){return  new R(codeMsg);}
	
	public static R error(CodeMsg codeMsg) {
		return new R(codeMsg);
	}

	/**
	 * 返回错误信息，使用默认错误代码
	 *
	 * @param message 错误消息
	 * @return R
	 */
	public static R error(String message) {
		R r = new R();
		r.clear();
		r.put("code", CodeMsg.SERVER_ERROR.getCode());
		r.put("msg", message);
		return r;
	}

	/**
	 * 返回错误信息，使用指定错误代码
	 *
	 * @param code 错误代码
	 * @param message 错误消息
	 * @return R
	 */
	public static R error(int code, String message) {
		R r = new R();
		r.clear();
		r.put("code", code);
		r.put("msg", message);
		return r;
	}

	public static R info(Object value) {

		R r = new R();
		r.clear();
		r.put("code",100) ;
		r.put("message", value);

		return r;
	}

	public static R warning(Object value) {

		R r = new R();
		r.clear();
		r.put("code",-1) ;
		r.put("warning", value);

		return r;
	}
	
	public static R ok(Object value) {
		
		R r = new R();
		r.put("data", value);
		
		return r;
	}



	public static R ok(Map<String, Object> map) {
		R r = new R();
		r.putAll(map);
		return r;
	}
	
	public static R ok() {
		return new R();
	}

	/**
	 * 返回操作结果，包含成功状态和消息
	 *
	 * @param success 操作是否成功
	 * @param message 操作消息
	 * @return R
	 */
	public static R ok(boolean success, String message) {
		R r = new R();
		r.put("success", success);
		r.put("message", message);
		return r;
	}

	@Override
	public R put(String key, Object value) {
		super.put(key, value);
		return this;
	}


}
