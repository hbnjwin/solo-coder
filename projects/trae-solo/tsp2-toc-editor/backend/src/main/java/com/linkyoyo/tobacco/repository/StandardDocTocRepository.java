package com.linkyoyo.tobacco.repository;

import com.cosium.spring.data.jpa.entity.graph.repository.EntityGraphJpaRepository;
import com.cosium.spring.data.jpa.entity.graph.repository.EntityGraphJpaSpecificationExecutor;
import com.linkyoyo.tobacco.entity.StandardDocToc;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.util.List;

/**
 * 标准文档目次仓库
 */
public interface StandardDocTocRepository extends EntityGraphJpaRepository<StandardDocToc, Integer>, EntityGraphJpaSpecificationExecutor<StandardDocToc> {

    /**
     * 根据标准文档ID查询目次
     * @param standardDocId 标准文档ID
     * @return 目次列表
     */
    List<StandardDocToc> findByStandardDocIdOrderByDisplayOrder(Integer standardDocId);

    /**
     * 根据标准文档ID和章节编号查询目次
     * @param standardDocId 标准文档ID
     * @param sectionNumber 章节编号
     * @return 目次
     */
    StandardDocToc findByStandardDocIdAndSectionNumber(Integer standardDocId, String sectionNumber);

    /**
     * 根据标准文档ID删除所有目次
     * @param standardDocId 标准文档ID
     */
    void deleteByStandardDocId(Integer standardDocId);


}
