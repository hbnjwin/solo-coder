package com.linkyoyo.tobacco.service;
import com.linkyoyo.tobacco.entity.StandardDoc;
import com.linkyoyo.tobacco.info.StandardDocInfo;
import com.linkyoyo.tobacco.info.PageInfo;
import com.linkyoyo.tobacco.info.StandardDocDetailResponse;
import com.linkyoyo.tobacco.info.UploadSingleFileInfo;
import com.linkyoyo.tobacco.info.UploadZipFileInfo;
import com.linkyoyo.tobacco.query.StandardDocQuery;
import com.linkyoyo.tobacco.result.R;
import org.springframework.web.multipart.MultipartFile;


import java.io.File;
import java.io.IOException;
import java.util.List;

public interface StandardDocService {
    PageInfo<StandardDoc> getStandardDocList(StandardDocQuery standardDocQuery);

    R queryByCondition(StandardDocQuery standardDocQuery);
    
    /**
     * 高级查询标准文档
     * 根据JSON条件字符串进行查询，支持多种查询条件
     * 
     * @param standardDocQuery 包含condition字段的查询对象
     * @return 查询结果，包含分页信息
     */
    R advancedQuery(StandardDocQuery standardDocQuery);
    R createOrUpdate(StandardDocInfo standardDocInfo) ;
    StandardDocInfo getStandardDocDetail(Integer id) ;

    R uploadZip(UploadZipFileInfo uploadZipInfo);

    R uploadFile(UploadSingleFileInfo uploadZipInfo) throws IOException;

    File getFile(Integer id);

    /**
     *
     * status 设置标准文档状态 1:现行有效  -1:已经废止
     *
     * id  标准文档id
     * @return
     */
    R setStandardDocStatus(Integer id,Integer status);

    PageInfo<StandardDoc> getListByEnterprise(StandardDocQuery standardDocQuery);


    PageInfo<StandardDoc> getListByEnterpriseAll(StandardDocQuery standardDocQuery);

    R deleteDoc(Integer id);


    File getSumFile(Integer deptId);

    File getDetailFile(Integer deptId,Integer systemId);

    /**
     * 根据ID获取标准文档详情，包含标准文档、目录和内容
     *
     * @param id 标准文档ID
     * @return 标准文档详情响应
     */
    StandardDocDetailResponse getStandardDocWithTocAndContent(Integer id);

    /**
     * 更新标准文档
     *
     * @param standardDoc 标准文档
     * @return 更新后的标准文档
     */
    StandardDoc updateStandardDoc(StandardDoc standardDoc);

    /**
     * 为单个标准文档生成并保存XML结构化内容
     *
     * @param id 标准文档ID
     * @return 执行结果
     */
    R generateAndSaveXmlContent(Integer id);

    /**
     * 批量为标准文档生成并保存XML结构化内容
     *
     * @param ids 标准文档ID列表，为空时处理所有没有XML内容的文档
     * @return 执行结果，包含成功和失败数量
     */
    R batchGenerateAndSaveXmlContent(List<Integer> ids);
}
