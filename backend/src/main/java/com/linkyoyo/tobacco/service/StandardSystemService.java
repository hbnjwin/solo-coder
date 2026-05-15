package com.linkyoyo.tobacco.service;
import com.linkyoyo.tobacco.entity.StandardSystem;
import com.linkyoyo.tobacco.info.StandardSystemInfo;
import com.linkyoyo.tobacco.info.PageInfo;
import com.linkyoyo.tobacco.query.StandardSystemQuery;
import com.linkyoyo.tobacco.result.R;


import java.lang.reflect.InvocationTargetException;
import java.util.List;

public interface StandardSystemService {
    PageInfo<StandardSystem> getStandardSystemList(StandardSystemQuery standardSystemQuery);
    StandardSystem createOrUpdate(StandardSystemInfo standardSystemInfo) ;
    StandardSystemInfo getStandardSystemDetail(Integer id) ;

    List getTree() throws InvocationTargetException, IllegalAccessException, InstantiationException;




    R getTreeExclude(Integer deptId,Integer maxLevel);
}
