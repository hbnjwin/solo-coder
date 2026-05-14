package com.linkyoyo.tobacco.service;

import com.linkyoyo.tobacco.dto.StandardReferenceResponse;
import com.linkyoyo.tobacco.entity.StandardDoc;
import com.linkyoyo.tobacco.entity.StandardDocToc;
import com.linkyoyo.tobacco.entity.StandardNormativeReference;
import com.linkyoyo.tobacco.entity.neo4j.EnhancedStandardNode;
import com.linkyoyo.tobacco.entity.neo4j.NormativeReferenceNode;
import com.linkyoyo.tobacco.entity.neo4j.PersonNode;
import com.linkyoyo.tobacco.entity.neo4j.StandardDepartmentNode;
import com.linkyoyo.tobacco.entity.neo4j.StandardLevelNode;
import com.linkyoyo.tobacco.entity.neo4j.SystemNode;
import com.linkyoyo.tobacco.entity.neo4j.TocNode;
import com.linkyoyo.tobacco.entity.neo4j.UnitNode;

import java.util.List;
import java.util.Map;

/**
 * 增强的标准文档Neo4j服务接口
 */
public interface EnhancedStandardDocNeo4jService {
    /**
     * 导入标准文档及其目录和规范性引用文件到Neo4j
     * @param standardDoc 标准文档
     * @param tocList 目录列表
     * @param mdContent Markdown内容
     * @param normativeReferences 规范性引用文件列表
     * @return 创建的标准节点
     */
    EnhancedStandardNode importStandardDocWithToc(StandardDoc standardDoc, List<StandardDocToc> tocList,
                                                String mdContent, List<StandardNormativeReference> normativeReferences);

    /**
     * 根据标准ID查询完整的标准文档信息
     * @param standardId 标准文档ID
     * @return 标准节点
     */
    EnhancedStandardNode findFullStandardDoc(String standardId);

    /**
     * 查询标准文档的简化信息，仅返回指定的属性和关系节点
     * @param standardId 标准ID
     * @return 包含指定属性和关系节点的标准节点
     */
    EnhancedStandardNode findSimplifiedStandardDoc(String standardId);

    /**
     * 根据标准号查询标准文档
     * @param standardNo 标准号
     * @return 标准节点
     */
    EnhancedStandardNode findByStandardNo(String standardNo);

    /**
     * 根据标准号和部门名称查询标准文档
     * @param standardNo 标准号
     * @param deptName 部门名称
     * @return 标准节点
     */
    EnhancedStandardNode findByStandardNoAndDeptName(String standardNo, String deptName);

    /**
     * 根据起草单位查询标准文档
     * @param draftUnit 起草单位
     * @return 标准节点列表
     */
    List<EnhancedStandardNode> findByDraftUnit(String draftUnit);

    /**
     * 根据发布单位查询标准文档
     * @param publishUnit 发布单位
     * @return 标准节点列表
     */
    List<EnhancedStandardNode> findByPublishUnit(String publishUnit);

    /**
     * 根据标准级别查询标准文档
     * @param standardLevelName 标准级别
     * @return 标准节点列表
     */
    List<EnhancedStandardNode> findByStandardLevel(String standardLevelName);

    /**
     * 查找标准文档的规范性引用文件
     * @param standardId 标准文档ID
     * @return 规范性引用文件节点列表
     */
    List<NormativeReferenceNode> findNormativeReferences(String standardId);

    /**
     * 查找引用了特定标准的文档
     * @param standardNo 标准号
     * @return 引用该标准的文档节点列表
     */
    List<EnhancedStandardNode> findReferencingStandards(String standardNo);

    /**
     * 获取标准文档的目录结构
     * @param standardId 标准文档ID
     * @return 目录节点列表
     */
    List<TocNode> findTableOfContents(String standardId);

    /**
     * 获取规范性引用文件的统计信息
     * @return 统计信息
     */
    Map<String, Object> getReferenceStatistics();

    /**
     * 根据关键字搜索标准文档
     * @param keyword 关键字
     * @return 匹配的标准节点列表
     */
    List<EnhancedStandardNode> searchByKeyword(String keyword);

    /**
     * 根据规范性引用文件搜索标准文档
     * @param referenceStandardNo 引用的标准号
     * @param referenceType 引用类型
     * @return 匹配的标准节点列表
     */
    List<EnhancedStandardNode> searchByReference(String referenceStandardNo, String referenceType);

    /**
     * 批量导入标准文档
     * @param standardIds 标准文档ID列表
     * @return 导入成功的数量
     */
    int batchImportStandards(List<Integer> standardIds);

    /**
     * 根据标准体系查询标准文档
     * @param systemName 标准体系名称
     * @return 标准节点列表
     */
    List<EnhancedStandardNode> findBySystemName(String systemName);

    /**
     * 根据归属分标委查询标准文档
     * @param departmentName 归属分标委名称
     * @return 标准节点列表
     */
    List<EnhancedStandardNode> findByDepartmentName(String departmentName);

    /**
     * 根据起草人查询标准文档
     * @param drafterName 起草人姓名
     * @return 标准节点列表
     */
    List<EnhancedStandardNode> findByDrafter(String drafterName);

    /**
     * 获取所有标准体系
     * @return 标准体系节点列表
     */
    List<SystemNode> findAllSystems();

    /**
     * 获取所有标准级别
     * @return 标准级别节点列表
     */
    List<StandardLevelNode> findAllStandardLevels();

    /**
     * 获取所有归属分标委
     * @return 归属分标委节点列表
     */
    List<StandardDepartmentNode> findAllDepartments();

    /**
     * 获取所有单位
     * @return 单位节点列表
     */
    List<UnitNode> findAllUnits();

    /**
     * 获取所有起草人
     * @return 人员节点列表
     */
    List<PersonNode> findAllDrafters();

    /**
     * 查询标准的引用关系
     * @param standardId 标准文档ID
     * @return 标准引用关系响应
     */
    StandardReferenceResponse findStandardReferences(String standardId);

    /**
     * 查询标准文档的JSON格式简化信息，返回指定的属性和关系节点
     * @param standardId 标准ID
     * @return 包含属性和关系节点的JSON格式数据
     */
    Map<String, Object> findStandardDocAsJson(String standardId);
}
