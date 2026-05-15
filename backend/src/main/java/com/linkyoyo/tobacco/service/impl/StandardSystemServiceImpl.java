package com.linkyoyo.tobacco.service.impl;

import cn.hutool.core.collection.CollectionUtil;
import com.github.wenhao.jpa.PredicateBuilder;
import com.github.wenhao.jpa.Specifications;
import com.linkyoyo.tobacco.entity.*;
import com.linkyoyo.tobacco.info.StandardEvaluateScoreInfo;
import com.linkyoyo.tobacco.info.StandardSystemInfo;
import com.linkyoyo.tobacco.info.PageInfo;
import com.linkyoyo.tobacco.query.GenerateTree;
import com.linkyoyo.tobacco.query.StandardSystemQuery;
import com.linkyoyo.tobacco.repository.StandardSystemRepository;
import com.linkyoyo.tobacco.result.R;
import com.linkyoyo.tobacco.service.StandardSystemService;
import com.linkyoyo.tobacco.support.CommonFunc;
import com.linkyoyo.tobacco.support.GenerateTreeData;
import com.linkyoyo.tobacco.support.Node;
import com.linkyoyo.tobacco.util.PageableUtil;
import com.querydsl.jpa.impl.JPAQueryFactory;
import org.springframework.beans.BeanUtils;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.domain.Specification;
import org.springframework.stereotype.Service;

import javax.persistence.EntityManager;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.ParameterizedType;
import java.lang.reflect.Type;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.stream.Collectors;

@Service
public class StandardSystemServiceImpl implements StandardSystemService {
    @Autowired
    private JPAQueryFactory jpaQueryFactory;

    @Autowired
    private EntityManager entityManager;

    @Autowired
    private StandardSystemRepository standardSystemRepository;

    @Override
    public List  getTree() throws InvocationTargetException, IllegalAccessException, InstantiationException {

        QStandardSystem qStandardSystem = QStandardSystem.standardSystem;
        List<StandardSystem> lstStandardSystem = standardSystemRepository.findAll();
//                jpaQueryFactory.selectFrom(qStandardSystem).fetch();
        CollectionUtil.sort(lstStandardSystem, (o1, o2) -> o1.getId().compareTo(o2.getId())) ;
     /*   List<Node> lstNode = lstStandardSystem.stream().map(f -> Node.builder()
                .id((String) f.getSystemCode())
                .text((String) f.getSystemName())
                .parentId((String) f.getParentSystemCode()).build()).collect(Collectors.toList());*/

        List<StandardSystemInfo> lstNode =  lstStandardSystem.stream().map(f ->
        {   StandardSystemInfo g=new StandardSystemInfo();
            BeanUtils.copyProperties(f, g);
            return g;
        }).collect(Collectors.toList());

        List<StandardSystemInfo>  bakLstNode = new ArrayList<>();
        bakLstNode.addAll(lstNode)  ;
        List<StandardSystemInfo> root = new ArrayList<>() ;

        GenerateTree<StandardSystemInfo> generateTree = new GenerateTree<StandardSystemInfo>();
        generateTree.setId("systemCode");
        generateTree.setName("systemName");
        generateTree.setParentId("parentSystemCode");
        generateTree.setChildren("children");
        GenerateTreeData generateTreeData = new GenerateTreeData<StandardSystemInfo>(generateTree);

        generateTreeData.generateTree(lstNode,bakLstNode, root);


        return root;


    }
    @Override
    public PageInfo<StandardSystem> getStandardSystemList(StandardSystemQuery standardSystemQuery) {
        Pageable pageable = PageableUtil.build(standardSystemQuery);

        return PageableUtil.info(standardSystemRepository.findAll(CommonFunc.<StandardSystem>getWhere(standardSystemQuery), pageable));
    }

    @Override
    public StandardSystem createOrUpdate(StandardSystemInfo standardSystemInfo) {
        if (Objects.isNull(standardSystemInfo.getId())) {
            StandardSystem standardSystem = StandardSystem.builder().build();
            BeanUtils.copyProperties(standardSystemInfo, standardSystem);
            standardSystem = standardSystemRepository.save(standardSystem);

            return standardSystem;
        } else {
            QStandardSystem qStandardSystem = QStandardSystem.standardSystem;
            StandardSystem standardSystem = jpaQueryFactory.select(qStandardSystem).from(qStandardSystem).where(qStandardSystem.id.eq(standardSystemInfo.getId())).fetchOne();
            BeanUtils.copyProperties(standardSystemInfo, standardSystem);
            standardSystem = standardSystemRepository.save(standardSystem);    
       
            return standardSystem;
        }
    }

    @Override
    public StandardSystemInfo getStandardSystemDetail(Integer id) {
        QStandardSystem qStandardSystem = QStandardSystem.standardSystem;
        StandardSystem standardSystem = jpaQueryFactory.select(qStandardSystem).from(qStandardSystem).where(qStandardSystem.id.eq(id)).fetchOne();
        StandardSystemInfo standardSystemInfo = new StandardSystemInfo();
        BeanUtils.copyProperties(standardSystem, standardSystemInfo);
        
        return standardSystemInfo;
    }

    @Override
    public R getTreeExclude(Integer deptId ,Integer maxLevel){
        QStandardSystem qStandardSystem = QStandardSystem.standardSystem;
        CommonFunc.clearEntityManager(entityManager);
        List<StandardSystem> lstStandardSystem =
//                jpaQueryFactory.select(qStandardSystem).from(qStandardSystem).where(qStandardSystem.excludeDeptId.contains(","+deptId.toString()+",").not()
//                .and(qStandardSystem.systemLevel.loe(maxLevel))).fetch();

        standardSystemRepository.findAll(Specifications.<StandardSystem>and().le("systemLevel",maxLevel)
                .notLike("excludeDeptId","%,"+deptId.toString()+",%").build()) ;


        List<StandardSystemInfo> lstNode =  lstStandardSystem.stream().map(f ->
        {   StandardSystemInfo g=new StandardSystemInfo();
            BeanUtils.copyProperties(f, g);
            return g;
        }).collect(Collectors.toList());

        CollectionUtil.sort(lstNode, (a,b)->a.getSystemCode().compareTo(b.getSystemCode()));

        List<StandardSystemInfo>  bakLstNode = new ArrayList<>();
        bakLstNode.addAll(lstNode)  ;
        List<StandardSystemInfo> root = new ArrayList<>() ;

        GenerateTree<StandardSystemInfo> generateTree = new GenerateTree<StandardSystemInfo>();
        generateTree.setId("systemCode");
        generateTree.setName("systemName");
        generateTree.setParentId("parentSystemCode");
        generateTree.setChildren("children");
        GenerateTreeData generateTreeData = new GenerateTreeData<StandardSystemInfo>(generateTree);

        try {
            generateTreeData.generateTree(lstNode, bakLstNode, root);
        }
        catch (Exception e) {
            return R.warning("生成树失败");
        }

        return R.ok(root);




    }
}
